use serde::Deserialize;

use crate::models::{common::empty_string_as_none, measurements::UnitType};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Temperatures {
    #[serde(default)]
    pub text_summary: Option<String>,
    #[serde(rename = "temperature", default)]
    pub temperature: Vec<Temperature>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    #[serde(rename = "$text", default)]
    pub value: Option<f64>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<TemperatureUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@class", default, deserialize_with = "empty_string_as_none")]
    pub class: Option<TemperatureClass>,
    #[serde(rename = "@year", default)]
    pub year: Option<i32>,
    #[serde(rename = "@period", default, deserialize_with = "empty_string_as_none")]
    pub period: Option<String>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum TemperatureUnit {
    C,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum TemperatureClass {
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "dewpoint")]
    Dewpoint,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "extremeMax")]
    ExtremeMax,
    #[serde(rename = "extremeMin")]
    ExtremeMin,
    #[serde(rename = "normalMax")]
    NormalMax,
    #[serde(rename = "normalMin")]
    NormalMin,
    #[serde(rename = "normalMean")]
    NormalMean,
    #[serde(rename = "mean")]
    Mean,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_temperature() {
        let xml = r#"<temperature unitType="metric" units="C" qaValue="100">3.0</temperature>"#;
        let temperature: Temperature = from_str(xml).unwrap();

        assert_eq!(temperature.value, Some(3.0));
        assert_eq!(temperature.unit, Some(TemperatureUnit::C));
        assert_eq!(temperature.unit_type, Some(UnitType::Metric));
        assert_eq!(temperature.class, None);
        assert_eq!(temperature.year, None);
        assert_eq!(temperature.period, None);
        assert_eq!(temperature.qa_value, Some(100));
    }
}
