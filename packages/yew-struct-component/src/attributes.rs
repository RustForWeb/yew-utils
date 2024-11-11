use std::{
    collections::HashMap,
    ops::Deref,
    option::{IntoIter, Iter, IterMut},
};

use yew::{html::IntoPropValue, AttrValue};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes(Option<HashMap<AttrValue, Option<AttrValue>>>);

impl Attributes {
    pub fn with_defaults<I: Into<Attributes>>(mut self, defaults: I) -> Attributes {
        let defaults: Attributes = defaults.into();

        self.0 = match (self.0, defaults.0) {
            (Some(map), Some(defaults)) => Some(defaults.into_iter().chain(map).collect()),
            (Some(map), None) => Some(map),
            (None, Some(defaults)) => Some(defaults),
            (None, None) => None,
        };

        self
    }
}

impl Deref for Attributes {
    type Target = Option<HashMap<AttrValue, Option<AttrValue>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<AttrValue, Option<AttrValue>>> for Attributes {
    fn from(value: HashMap<AttrValue, Option<AttrValue>>) -> Attributes {
        Attributes(Some(value))
    }
}

impl From<HashMap<AttrValue, AttrValue>> for Attributes {
    fn from(value: HashMap<AttrValue, AttrValue>) -> Attributes {
        Attributes(Some(
            value
                .into_iter()
                .map(|(key, value)| (key, Some(value)))
                .collect(),
        ))
    }
}

impl From<HashMap<String, Option<String>>> for Attributes {
    fn from(value: HashMap<String, Option<String>>) -> Attributes {
        Attributes(Some(
            value
                .into_iter()
                .map(|(key, value)| (AttrValue::from(key), value.map(AttrValue::from)))
                .collect(),
        ))
    }
}

impl From<HashMap<String, String>> for Attributes {
    fn from(value: HashMap<String, String>) -> Attributes {
        Attributes(Some(
            value
                .into_iter()
                .map(|(key, value)| (AttrValue::from(key), Some(AttrValue::from(value))))
                .collect(),
        ))
    }
}

impl<const N: usize> From<[(AttrValue, Option<AttrValue>); N]> for Attributes {
    fn from(value: [(AttrValue, Option<AttrValue>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value)))
    }
}

impl<const N: usize> From<[(AttrValue, AttrValue); N]> for Attributes {
    fn from(value: [(AttrValue, AttrValue); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(
            value.map(|(key, value)| (key, Some(value))),
        )))
    }
}

impl<const N: usize> From<[(&str, Option<&str>); N]> for Attributes {
    fn from(value: [(&str, Option<&str>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                value.map(|value| AttrValue::from(value.to_string())),
            )
        }))))
    }
}

impl<const N: usize> From<[(&str, &str); N]> for Attributes {
    fn from(value: [(&str, &str); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                Some(AttrValue::from(value.to_string())),
            )
        }))))
    }
}

impl<const N: usize> From<[(&str, Option<String>); N]> for Attributes {
    fn from(value: [(&str, Option<String>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (AttrValue::from(key.to_string()), value.map(AttrValue::from))
        }))))
    }
}

impl<const N: usize> From<[(&str, String); N]> for Attributes {
    fn from(value: [(&str, String); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (
                AttrValue::from(key.to_string()),
                Some(AttrValue::from(value)),
            )
        }))))
    }
}

impl<const N: usize> From<[(String, Option<String>); N]> for Attributes {
    fn from(value: [(String, Option<String>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (AttrValue::from(key), value.map(AttrValue::from))
        }))))
    }
}

impl<const N: usize> From<[(String, String); N]> for Attributes {
    fn from(value: [(String, String); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (AttrValue::from(key), Some(AttrValue::from(value)))
        }))))
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
        self.into()
    }
}

impl IntoPropValue<Attributes> for HashMap<AttrValue, AttrValue> {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl IntoPropValue<Attributes> for HashMap<String, Option<String>> {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl IntoPropValue<Attributes> for HashMap<String, String> {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, Option<AttrValue>); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(AttrValue, AttrValue); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, Option<&str>); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, &str); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, Option<String>); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(&str, String); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(String, Option<String>); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Attributes> for [(String, String); N] {
    fn into_prop_value(self) -> Attributes {
        self.into()
    }
}
