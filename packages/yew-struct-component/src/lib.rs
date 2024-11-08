//! Define [Yew](https://yew.rs/) components using structs.

use std::{collections::HashMap, ops::Deref};

use yew::{html::IntoPropValue, AttrValue};
pub use yew_struct_component_macro::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Attributes(HashMap<AttrValue, Option<AttrValue>>);

impl Deref for Attributes {
    type Target = HashMap<AttrValue, Option<AttrValue>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoPropValue<Attributes> for HashMap<AttrValue, Option<AttrValue>> {
    fn into_prop_value(self) -> Attributes {
        Attributes(self)
    }
}

impl IntoPropValue<Attributes> for HashMap<AttrValue, AttrValue> {
    fn into_prop_value(self) -> Attributes {
        Attributes(
            self.into_iter()
                .map(|(key, value)| (key, Some(value)))
                .collect(),
        )
    }
}

impl IntoPropValue<Attributes> for HashMap<String, Option<String>> {
    fn into_prop_value(self) -> Attributes {
        Attributes(
            self.into_iter()
                .map(|(key, value)| (AttrValue::from(key), value.map(AttrValue::from)))
                .collect(),
        )
    }
}

impl IntoPropValue<Attributes> for HashMap<String, String> {
    fn into_prop_value(self) -> Attributes {
        Attributes(
            self.into_iter()
                .map(|(key, value)| (AttrValue::from(key), Some(AttrValue::from(value))))
                .collect(),
        )
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, Option<AttrValue>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(self))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, AttrValue); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(
            self.map(|(key, value)| (key, Some(value))),
        ))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, Option<&str>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(self.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                value.map(|value| AttrValue::from(value.to_string())),
            )
        })))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, &str); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(self.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                Some(AttrValue::from(value.to_string())),
            )
        })))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(String, Option<String>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(self.map(|(key, value)| {
            (AttrValue::from(key), value.map(AttrValue::from))
        })))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(String, String); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(HashMap::from_iter(self.map(|(key, value)| {
            (AttrValue::from(key), Some(AttrValue::from(value)))
        })))
    }
}
