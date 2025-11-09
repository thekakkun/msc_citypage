use serde::Deserialize;

use crate::models::{
    common::{deserialize_some_f64, empty_string_as_none},
    measurements::UnitType,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pressures {
    #[serde(default)]
    pub text_summary: Option<String>,
    pub pressure: PressureForecast,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PressureForecast {
    #[serde(rename = "$text", default)]
    pub value: Option<f64>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<PressureUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PressureCondition {
    #[serde(rename = "$text", default)]
    pub value: Option<f64>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<PressureUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@change", default, deserialize_with = "deserialize_some_f64")]
    pub change: Option<f64>,
    #[serde(
        rename = "@tendency",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub tendency: Option<PressureTendency>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum PressureUnit {
    #[serde(rename = "kPa")]
    KPa,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum PressureTendency {
    #[serde(rename = "rising", alias = "à la hausse")]
    Rising,
    #[serde(rename = "falling", alias = "à la baisse")]
    Falling,
    #[serde(rename = "steady", alias = "stationnaire")]
    Steady,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_pressure_condition_en() {
        let xml =
            r#"<pressure unitType="metric" units="kPa" change="" qaValue="100">100.2</pressure>"#;
        let pressure_condition: PressureCondition = from_str(xml).unwrap();

        assert_eq!(pressure_condition.value, Some(100.2));
        assert_eq!(pressure_condition.unit, Some(PressureUnit::KPa));
        assert_eq!(pressure_condition.unit_type, Some(UnitType::Metric));
        assert_eq!(pressure_condition.change, None);
        assert_eq!(pressure_condition.tendency, None);
        assert_eq!(pressure_condition.qa_value, Some(100));
    }

    #[test]
    fn test_deserialize_pressure_condition_fr() {
        let xml = r#"<pressure unitType="metric" units="kPa" change="0.02" tendency="à la hausse" qaValue="100">101.4</pressure>"#;
        let pressure_condition: PressureCondition = from_str(xml).unwrap();

        assert_eq!(pressure_condition.value, Some(101.4));
        assert_eq!(pressure_condition.unit, Some(PressureUnit::KPa));
        assert_eq!(pressure_condition.unit_type, Some(UnitType::Metric));
        assert_eq!(pressure_condition.change, Some(0.02));
        assert_eq!(pressure_condition.tendency, Some(PressureTendency::Rising));
        assert_eq!(pressure_condition.qa_value, Some(100));
    }
}
