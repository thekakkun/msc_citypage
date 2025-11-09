use serde::Deserialize;

use crate::models::common::empty_string_as_none;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RelativeHumidity {
    #[serde(rename = "$text", default)]
    pub value: Option<f32>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<RelativeHumidityUnit>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum RelativeHumidityUnit {
    #[serde(rename = "%")]
    Percent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_relative_humidity() {
        let xml = r#"<relativeHumidity units="%" qaValue="100">99</relativeHumidity>"#;
        let relative_humidity: RelativeHumidity = from_str(xml).unwrap();

        assert_eq!(relative_humidity.value, Some(99.0));
        assert_eq!(relative_humidity.unit, Some(RelativeHumidityUnit::Percent));
        assert_eq!(relative_humidity.qa_value, Some(100));
    }
}
