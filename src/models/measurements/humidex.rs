use serde::Deserialize;

use crate::models::{common::empty_string_as_none, measurements::UnitType};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Humidex {
    #[serde(rename = "calculated", default)]
    pub calculated: Vec<CalculatedHumidex>,
    #[serde(default)]
    pub text_summary: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CalculatedHumidex {
    #[serde(rename = "$text", default)]
    pub value: Option<f32>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_calculated_humidex() {
        let xml = r#"<humidex unitType="metric">24</humidex>"#;
        let humidex: CalculatedHumidex = from_str(xml).unwrap();

        assert_eq!(humidex.value, Some(24.0));
        assert_eq!(humidex.unit_type, Some(UnitType::Metric));
        assert_eq!(humidex.qa_value, None);
    }
}
