use serde::Deserialize;

use crate::models::{common::empty_string_as_none, measurements::UnitType};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct VisibilityCondition {
    #[serde(rename = "$text", default)]
    pub value: Option<f64>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<VisibilityUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    #[serde(default)]
    pub wind_visib: Option<VisibilitySubForecast>,
    #[serde(default)]
    pub other_visib: Option<VisibilitySubForecast>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VisibilitySubForecast {
    #[serde(default)]
    pub text_summary: Option<String>,
    #[serde(rename = "@cause", default, deserialize_with = "empty_string_as_none")]
    pub cause: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum VisibilityUnit {
    #[serde(rename = "km")]
    Km,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_visibility_condition() {
        let xml = r#"<visibility unitType="metric" units="km" qaValue="100">16.1</visibility>"#;
        let visibility: VisibilityCondition = from_str(xml).unwrap();

        assert_eq!(visibility.value, Some(16.1));
        assert_eq!(visibility.unit, Some(VisibilityUnit::Km));
        assert_eq!(visibility.unit_type, Some(UnitType::Metric));
        assert_eq!(visibility.qa_value, Some(100));
    }
}
