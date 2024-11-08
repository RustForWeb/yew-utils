//! Define [Yew](https://yew.rs/) components using structs.

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, AttrStyle, Attribute, Data, DeriveInput, Ident, LitBool,
    LitStr, Meta, Type,
};

#[derive(Debug, Default)]
struct StructComponentAttrArgs {
    tag: Option<String>,
    dynamic_tag: Option<bool>,
    no_children: Option<bool>,
}

fn parse_struct_component_attr(attr: &Attribute) -> Result<StructComponentAttrArgs, syn::Error> {
    if !matches!(attr.style, AttrStyle::Outer) {
        Err(syn::Error::new(attr.span(), "not an inner attribute"))
    } else if let Meta::List(list) = &attr.meta {
        let mut args = StructComponentAttrArgs::default();

        list.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let value = meta.value().and_then(|value| value.parse::<LitStr>())?;

                args.tag = Some(value.value());

                Ok(())
            } else if meta.path.is_ident("dynamic_tag") {
                let value = meta.value().and_then(|value| value.parse::<LitBool>())?;

                args.dynamic_tag = Some(value.value());

                Ok(())
            } else if meta.path.is_ident("no_children") {
                let value = meta.value().and_then(|value| value.parse::<LitBool>())?;

                args.no_children = Some(value.value());

                Ok(())
            } else {
                Err(meta.error("unknown property"))
            }
        })?;

        Ok(args)
    } else {
        Err(syn::Error::new(attr.span(), "not a list"))
    }
}

#[proc_macro_attribute]
pub fn struct_component(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(StructComponent, attributes(struct_component))]
pub fn derive_struct_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let mut args = StructComponentAttrArgs::default();
    for attr in &derive_input.attrs {
        if attr.path().is_ident("struct_component") {
            match parse_struct_component_attr(attr) {
                Ok(result) => {
                    args = result;
                }
                Err(error) => {
                    return error.to_compile_error().into();
                }
            }
        }
    }

    if let Data::Struct(data_struct) = &derive_input.data {
        let ident = derive_input.ident.clone();

        let mut attributes: Vec<TokenStream> = vec![];
        let mut attribute_checked: Option<TokenStream> = None;
        let mut attribute_value: Option<TokenStream> = None;
        let mut listeners: Vec<Ident> = vec![];
        let mut attributes_map: Option<TokenStream> = None;
        let mut tag: Option<TokenStream> = None;
        let mut node_ref: Option<TokenStream> = None;

        for field in &data_struct.fields {
            if let Some(ident) = &field.ident {
                if let Some(attr) = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("struct_component"))
                {
                    match parse_struct_component_attr(attr) {
                        Ok(args) => {
                            if args.dynamic_tag.is_some_and(|dynamic_tag| dynamic_tag) {
                                tag = Some(quote! {
                                    self.#ident.to_string()
                                });

                                continue;
                            }
                        }
                        Err(error) => {
                            return error.to_compile_error().into();
                        }
                    }
                }

                if ident == "attributes" {
                    attributes_map = Some(quote! {
                        .chain(
                            self.attributes
                                .into_iter()
                                .flatten()
                                .flat_map(|(key, value)| value.map(|value| (
                                    ::yew::virtual_dom::AttrValue::from(key),
                                    ::yew::virtual_dom::AttributeOrProperty::Attribute(AttrValue::from(value)),
                                )),
                            ),
                        )
                    });

                    continue;
                }

                if ident == "node_ref" {
                    node_ref = Some(quote! {
                        tag.node_ref = self.node_ref;
                    });

                    continue;
                }

                if ident.to_string().starts_with("on") {
                    if let Type::Path(path) = &field.ty {
                        let first = path.path.segments.first();
                        if first.is_some_and(|segment| segment.ident == "Callback") {
                            listeners.push(ident.clone());

                            continue;
                        }
                    }
                }

                if ident == "checked" {
                    attribute_checked = Some(quote! {
                        tag.set_checked(self.checked);
                    });
                }

                if ident == "value" {
                    attribute_value = Some(quote! {
                        tag.set_value(self.value.clone());
                    });
                }

                match &field.ty {
                    Type::Path(path) => {
                        let name = ident.to_string().replace("_", "-");
                        let name = if name.starts_with("r#") {
                            name.strip_prefix("r#").expect("String should have prefix.")
                        } else {
                            name.as_str()
                        }
                        .to_token_stream();

                        let first = path.path.segments.first();

                        attributes.push(
                            if first.is_some_and(|segment| segment.ident == "Option") {
                                quote! {
                                    self.#ident.map(|value| (
                                        ::yew::virtual_dom::AttrValue::from(#name),
                                        ::yew::virtual_dom::AttributeOrProperty::Attribute(
                                            ::yew::virtual_dom::AttrValue::from(value)
                                        )
                                    ))
                                }
                            } else if first.is_some_and(|segment| segment.ident == "bool") {
                                quote! {
                                    self.#ident.then_some((
                                        ::yew::virtual_dom::AttrValue::from(#name),
                                        ::yew::virtual_dom::AttributeOrProperty::Attribute(
                                            ::yew::virtual_dom::AttrValue::from("")
                                        )
                                    ))
                                }
                            } else {
                                quote! {
                                    Some((
                                        ::yew::virtual_dom::AttrValue::from(#name),
                                        ::yew::virtual_dom::AttributeOrProperty::Attribute(
                                            ::yew::virtual_dom::AttrValue::from(self.#ident)
                                        )
                                    ))
                                }
                            },
                        );
                    }
                    _ => {
                        return syn::Error::new(field.ty.span(), "expected type path")
                            .to_compile_error()
                            .into()
                    }
                }
            }
        }

        let tag = if let Some(tag) =
            tag.or_else(|| args.tag.map(|tag| tag.as_str().to_token_stream()))
        {
            tag
        } else {
            return syn::Error::new(derive_input.span(), "`#[struct_component(tag = \"\")] or #[struct_component(dynamic_tag = true)]` is required")
                    .to_compile_error()
                    .into();
        };

        let arguments = if args.no_children.unwrap_or(false) {
            quote! {
                self
            }
        } else {
            quote! {
                self, children: ::yew::prelude::Html
            }
        };

        let children = (!args.no_children.unwrap_or(false)).then(|| {
            quote! {
                tag.add_child(children);
            }
        });

        quote! {
            impl #ident {
                pub fn render(#arguments) -> ::yew::prelude::Html {
                    let mut tag = ::yew::virtual_dom::VTag::new(#tag);
                    #node_ref

                    tag.set_attributes(::yew::virtual_dom::Attributes::IndexMap(
                        ::std::rc::Rc::new(
                            [
                                #(#attributes,)*
                            ]
                            .into_iter()
                            .flatten()
                            #attributes_map
                            .collect(),
                        ),
                    ));

                    #attribute_checked
                    #attribute_value

                    tag.set_listeners(::std::boxed::Box::new([
                        #(::yew::html::#listeners::Wrapper::__macro_new(
                            self.#listeners,
                        ),)*
                    ]));

                    #children

                    tag.into()
                }
            }
        }
        .into()
    } else {
        syn::Error::new(derive_input.span(), "expected struct")
            .to_compile_error()
            .into()
    }
}
