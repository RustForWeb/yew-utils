//! Define [Yew](https://yew.rs/) components using structs.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, AttrStyle, Data, DeriveInput, LitStr, Meta, Type};

#[proc_macro_attribute]
pub fn struct_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_derive(StructComponent)]
pub fn derive_struct_component(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let mut tag = "".to_string();
    for attr in &derive_input.attrs {
        if !matches!(attr.style, AttrStyle::Outer) {
            continue;
        }

        if let Meta::List(list) = &attr.meta {
            if list.path.is_ident("struct_component") {
                if let Err(err) = list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("tag") {
                        let value = meta.value().and_then(|value| value.parse::<LitStr>())?;

                        tag = value.value();

                        Ok(())
                    } else {
                        Err(meta.error("unknown key"))
                    }
                }) {
                    return syn::Error::new(
                        derive_input.span(),
                        format!("expected tag in `#[struct_component]` attribute: {}", err),
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
    }

    if tag.is_empty() {
        return syn::Error::new(derive_input.span(), "expected non-empty tag")
            .to_compile_error()
            .into();
    }

    if let Data::Struct(data_struct) = derive_input.data {
        let ident = derive_input.ident;
        let tag = tag.as_str().to_token_stream();

        let fields = data_struct
            .fields
            .into_iter()
            .filter_map(|field| field.ident.clone().map(|ident| (ident, field)));

        let attributes = fields.clone().filter(|(ident, _)| {
            ident != "node_ref" && ident != "attributes" && !ident.to_string().starts_with("on")
        });
        let attribute_values = attributes.clone().map(|(ident, field)| match field.ty {
            Type::Path(path) => {
                let name = ident.to_string().replace("_", "-");
                let name = if name.starts_with("r#") {
                    name.strip_prefix("r#").expect("String should have prefix.")
                } else {
                    name.as_str()
                }
                .to_token_stream();

                let first = path.path.segments.first();

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
                }
            }
            _ => syn::Error::new(field.ty.span(), "expected type path").to_compile_error(),
        });

        let listeners = fields
            .clone()
            .filter(|(ident, _)| ident.to_string().starts_with("on"));
        let listener_idents = listeners.map(|(ident, _)| ident);

        let node_ref = fields
            .clone()
            .find(|(ident, _)| ident == "node_ref")
            .map(|_| {
                quote! {
                    tag.node_ref = self.node_ref;
                }
            });
        let attributes_map = fields.clone().find(|(ident, _)| ident == "attributes").map(|_| quote! {
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

        quote! {
            impl #ident {
                pub fn render(self, children: ::yew::prelude::Html) -> ::yew::prelude::Html {
                    let mut tag = ::yew::virtual_dom::VTag::new(#tag);
                    #node_ref

                    tag.set_attributes(::yew::virtual_dom::Attributes::IndexMap(
                        ::std::rc::Rc::new(
                            [
                                #(#attribute_values,)*
                            ]
                            .into_iter()
                            .flatten()
                            #attributes_map
                            .collect(),
                        ),
                    ));

                    tag.set_listeners(::std::boxed::Box::new([
                        #(::yew::html::#listener_idents::Wrapper::__macro_new(
                            self.#listener_idents,
                        ),)*
                    ]));

                    tag.add_child(children);

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
