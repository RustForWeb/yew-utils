use std::{
    collections::HashMap,
    ops::Deref,
    option::{IntoIter, Iter, IterMut},
};

use yew::{html::IntoPropValue, AttrValue};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes(Option<HashMap<AttrValue, Option<AttrValue>>>);

impl Deref for Attributes {
    type Target = Option<HashMap<AttrValue, Option<AttrValue>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a HashMap<AttrValue, Option<AttrValue>>;
    type IntoIter = Iter<'a, HashMap<AttrValue, Option<AttrValue>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Attributes {
    type Item = &'a mut HashMap<AttrValue, Option<AttrValue>>;
    type IntoIter = IterMut<'a, HashMap<AttrValue, Option<AttrValue>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for Attributes {
    type Item = HashMap<AttrValue, Option<AttrValue>>;
    type IntoIter = IntoIter<HashMap<AttrValue, Option<AttrValue>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoPropValue<Attributes> for HashMap<AttrValue, Option<AttrValue>> {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(self))
    }
}

impl IntoPropValue<Attributes> for HashMap<AttrValue, AttrValue> {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(
            self.into_iter()
                .map(|(key, value)| (key, Some(value)))
                .collect(),
        ))
    }
}

impl IntoPropValue<Attributes> for HashMap<String, Option<String>> {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(
            self.into_iter()
                .map(|(key, value)| (AttrValue::from(key), value.map(AttrValue::from)))
                .collect(),
        ))
    }
}

impl IntoPropValue<Attributes> for HashMap<String, String> {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(
            self.into_iter()
                .map(|(key, value)| (AttrValue::from(key), Some(AttrValue::from(value))))
                .collect(),
        ))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, Option<AttrValue>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self)))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, AttrValue); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(
            self.map(|(key, value)| (key, Some(value))),
        )))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, Option<&str>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                value.map(|value| AttrValue::from(value.to_string())),
            )
        }))))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, &str); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                Some(AttrValue::from(value.to_string())),
            )
        }))))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, Option<String>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self.map(|(key, value)| {
            (AttrValue::from(key.to_string()), value.map(AttrValue::from))
        }))))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, String); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                Some(AttrValue::from(value)),
            )
        }))))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(String, Option<String>); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self.map(|(key, value)| {
            (AttrValue::from(key), value.map(AttrValue::from))
        }))))
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(String, String); N] {
    fn into_prop_value(self) -> Attributes {
        Attributes(Some(HashMap::from_iter(self.map(|(key, value)| {
            (AttrValue::from(key), Some(AttrValue::from(value)))
        }))))
    }
}
