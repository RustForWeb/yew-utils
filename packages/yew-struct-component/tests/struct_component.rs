use std::fmt::{self, Display};

use yew::prelude::*;
use yew_struct_component::{struct_component, StructComponent};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BoxAs {
    #[default]
    Div,
    Span,
}

impl Display for BoxAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxAs::Div => "div",
                BoxAs::Span => "span",
            }
        )
    }
}

#[derive(PartialEq, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub r#as: BoxAs,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<BoxChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
pub struct BoxChildProps {
    #[struct_component(dynamic_tag = true)]
    pub r#as: BoxAs,
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
}

#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let child_props = BoxChildProps {
        r#as: props.r#as,
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
    };

    if let Some(as_child) = &props.as_child {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
