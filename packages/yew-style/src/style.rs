use std::{
    fmt::{self, Display},
    ops::Deref,
};

use indexmap::IndexMap;
use yew::{AttrValue, html::IntoPropValue};

fn style_map_to_string(map: &IndexMap<String, Option<String>>) -> String {
    map.iter()
        .filter_map(|(key, value)| {
            value
                .as_ref()
                .and_then(|value| (!value.is_empty()).then_some(format!("{key}: {value};")))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Debug, PartialEq)]
pub enum InnerStyle {
    String(String),
    Structured(IndexMap<String, Option<String>>),
}

impl InnerStyle {
    pub fn with_defaults<I: Into<InnerStyle>>(self, defaults: I) -> Self {
        let defaults: InnerStyle = defaults.into();

        match (self, defaults) {
            (Self::String(string), Self::String(default_string)) => {
                Self::String(format!("{default_string} {string}"))
            }
            (Self::String(string), Self::Structured(default_map)) => {
                Self::String(format!("{} {}", style_map_to_string(&default_map), string))
            }
            (Self::Structured(map), Self::String(default_string)) => {
                Self::String(format!("{} {}", default_string, style_map_to_string(&map)))
            }
            (Self::Structured(map), Self::Structured(default_map)) => {
                InnerStyle::Structured(default_map.into_iter().chain(map).collect())
            }
        }
    }
}

impl Display for InnerStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(string) => write!(f, "{string}"),
            Self::Structured(map) => write!(f, "{}", style_map_to_string(map),),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style(pub Option<InnerStyle>);

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults<I: Into<Self>>(self, defaults: I) -> Self {
        let defaults: Self = defaults.into();

        Style(match (self.0, defaults.0) {
            (Some(style), Some(defaults)) => Some(style.with_defaults(defaults)),
            (Some(style), None) => Some(style),
            (None, Some(defaults)) => Some(defaults),
            (None, None) => None,
        })
    }
}

impl Deref for Style {
    type Target = Option<InnerStyle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .as_ref()
                .map(|inner_style| inner_style.to_string())
                .unwrap_or_default(),
        )
    }
}

impl From<Option<&str>> for Style {
    fn from(value: Option<&str>) -> Style {
        Style(value.map(|value| InnerStyle::String(value.to_string())))
    }
}

impl From<Option<String>> for Style {
    fn from(value: Option<String>) -> Style {
        Style(value.map(InnerStyle::String))
    }
}

impl From<&str> for Style {
    fn from(value: &str) -> Style {
        Style(Some(InnerStyle::String(value.to_string())))
    }
}

impl From<String> for Style {
    fn from(value: String) -> Style {
        Style(Some(InnerStyle::String(value)))
    }
}

impl From<IndexMap<String, Option<String>>> for Style {
    fn from(value: IndexMap<String, Option<String>>) -> Style {
        Style(Some(InnerStyle::Structured(value)))
    }
}

impl From<IndexMap<String, String>> for Style {
    fn from(value: IndexMap<String, String>) -> Style {
        Style(Some(InnerStyle::Structured(
            value
                .into_iter()
                .map(|(key, value)| (key, Some(value)))
                .collect(),
        )))
    }
}

impl<const N: usize> From<[(&str, Option<&str>); N]> for Style {
    fn from(value: [(&str, Option<&str>); N]) -> Style {
        Style(Some(InnerStyle::Structured(IndexMap::from_iter(
            value.map(|(key, value)| (key.to_string(), value.map(|value| value.to_string()))),
        ))))
    }
}

impl<const N: usize> From<[(&str, &str); N]> for Style {
    fn from(value: [(&str, &str); N]) -> Style {
        Style(Some(InnerStyle::Structured(IndexMap::from_iter(
            value.map(|(key, value)| (key.to_string(), Some(value.to_string()))),
        ))))
    }
}

impl<const N: usize> From<[(&str, Option<String>); N]> for Style {
    fn from(value: [(&str, Option<String>); N]) -> Style {
        Style(Some(InnerStyle::Structured(IndexMap::from_iter(
            value.map(|(key, value)| (key.to_string(), value)),
        ))))
    }
}

impl<const N: usize> From<[(&str, String); N]> for Style {
    fn from(value: [(&str, String); N]) -> Style {
        Style(Some(InnerStyle::Structured(IndexMap::from_iter(
            value.map(|(key, value)| (key.to_string(), Some(value))),
        ))))
    }
}

impl<const N: usize> From<[(String, Option<String>); N]> for Style {
    fn from(value: [(String, Option<String>); N]) -> Style {
        Style(Some(InnerStyle::Structured(IndexMap::from_iter(value))))
    }
}

impl<const N: usize> From<[(String, String); N]> for Style {
    fn from(value: [(String, String); N]) -> Style {
        Style(Some(InnerStyle::Structured(IndexMap::from_iter(
            value.map(|(key, value)| (key, Some(value))),
        ))))
    }
}

impl IntoPropValue<Style> for Option<&str> {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl IntoPropValue<Style> for Option<String> {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl IntoPropValue<Style> for &str {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl IntoPropValue<Style> for String {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl IntoPropValue<Style> for IndexMap<String, Option<String>> {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl IntoPropValue<Style> for IndexMap<String, String> {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Style> for [(&str, Option<&str>); N] {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Style> for [(&str, &str); N] {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Style> for [(&str, Option<String>); N] {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Style> for [(&str, String); N] {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Style> for [(String, Option<String>); N] {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<Style> for [(String, String); N] {
    fn into_prop_value(self) -> Style {
        self.into()
    }
}

impl From<&InnerStyle> for AttrValue {
    fn from(value: &InnerStyle) -> Self {
        AttrValue::from(value.to_string())
    }
}

impl From<&Style> for AttrValue {
    fn from(value: &Style) -> Self {
        AttrValue::from(value.to_string())
    }
}

impl From<InnerStyle> for AttrValue {
    fn from(value: InnerStyle) -> Self {
        AttrValue::from(value.to_string())
    }
}

impl From<Style> for AttrValue {
    fn from(value: Style) -> Self {
        AttrValue::from(value.to_string())
    }
}

impl IntoPropValue<Option<AttrValue>> for Style {
    fn into_prop_value(self) -> Option<AttrValue> {
        self.as_ref().map(|value| value.into())
    }
}

impl IntoPropValue<AttrValue> for Style {
    fn into_prop_value(self) -> AttrValue {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!("", Style::default().to_string());

        assert_eq!("", Style::from(None::<String>).to_string());

        assert_eq!(
            "margin: 1rem; padding: 0.5rem;",
            Style::from(Some("margin: 1rem; padding: 0.5rem;")).to_string(),
        );

        assert_eq!(
            "margin: 1rem; padding: 0.5rem;",
            Style::from("margin: 1rem; padding: 0.5rem;").to_string(),
        );

        assert_eq!(
            "color: white; border: 1px solid black;",
            Style::from([
                ("color", Some("white")),
                ("background-color", None),
                ("border", Some("1px solid black")),
            ])
            .to_string()
        );

        assert_eq!(
            "color: white; background-color: gray; border: 1px solid black;",
            Style::from([
                ("color", "white"),
                ("background-color", "gray"),
                ("border", "1px solid black"),
            ])
            .to_string()
        );
    }

    #[test]
    fn test_with_defaults() {
        // String with string defaults
        assert_eq!(
            Style::from("pointer-events: none; color: red;"),
            Style::from("color: red;").with_defaults("pointer-events: none;"),
        );
        assert_eq!(
            Style::from("color: blue; color: red;"),
            Style::from("color: red;").with_defaults("color: blue;"),
        );

        // String with structured defaults
        assert_eq!(
            Style::from("pointer-events: none; color: red;"),
            Style::from("color: red;").with_defaults([("pointer-events", "none")]),
        );
        assert_eq!(
            Style::from("color: blue; color: red;"),
            Style::from("color: red;").with_defaults([("color", "blue")]),
        );

        // Structured with string defaults
        assert_eq!(
            Style::from("pointer-events: none; color: red;"),
            Style::from([("color", "red")]).with_defaults("pointer-events: none;"),
        );
        assert_eq!(
            Style::from("color: blue; color: red;"),
            Style::from([("color", "red")]).with_defaults("color: blue;"),
        );

        // Structured with structured defaults
        assert_eq!(
            Style::from([("pointer-events", "none"), ("color", "red")]),
            Style::from([("color", "red")]).with_defaults([("pointer-events", "none")]),
        );
        assert_eq!(
            Style::from([("color", "red")]),
            Style::from([("color", "red")]).with_defaults([("color", "blue")]),
        );

        // Optional in structured
        assert_eq!(
            Style::from([("color", Some("red"))]),
            Style::from([("color", Some("red"))]).with_defaults([("color", Some("blue"))]),
        );
        assert_eq!(
            Style::from([("color", None::<String>)]),
            Style::from([("color", None::<String>)]).with_defaults([("color", Some("blue"))]),
        );
        assert_eq!(
            Style::from([("color", Some("red"))]),
            Style::from([("color", Some("red"))]).with_defaults([("color", None::<String>)]),
        );
        assert_eq!(
            Style::from([("color", None::<String>)]),
            Style::from([("color", None::<String>)]).with_defaults([("color", None::<String>)]),
        );
    }
}
