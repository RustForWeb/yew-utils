//! Define [Yew](https://yew.rs/) components using structs.

use std::{collections::HashMap, ops::Deref};

use yew::{html::IntoPropValue, AttrValue};
pub use yew_struct_component_macro::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Attributes(HashMap<AttrValue, AttrValue>);

impl Deref for Attributes {
    type Target = HashMap<AttrValue, AttrValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoPropValue<Attributes> for HashMap<AttrValue, AttrValue> {
    fn into_prop_value(self) -> Attributes {
        Attributes(self)
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, AttrValue); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(self))
    }
}
