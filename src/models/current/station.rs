use serde::Deserialize;

use crate::models::common::empty_string_as_none;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Station {
    #[serde(rename = "@code", default, deserialize_with = "empty_string_as_none")]
    pub code: Option<String>,
    #[serde(rename = "@lat", default, deserialize_with = "empty_string_as_none")]
    pub lat: Option<String>,
    #[serde(rename = "@lon", default, deserialize_with = "empty_string_as_none")]
    pub lon: Option<String>,
    #[serde(
        rename = "@country",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub country: Option<String>,
    #[serde(rename = "$text", default, deserialize_with = "empty_string_as_none")]
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_station() {
        let xml = r#"<station code="ywk" lat="52.93N" lon="66.87W">Wabush Airport</station>"#;
        let station: Station = from_str(xml).unwrap();

        assert_eq!(station.code, Some(String::from("ywk")));
        assert_eq!(station.lat, Some(String::from("52.93N")));
        assert_eq!(station.lon, Some(String::from("66.87W")));
        assert_eq!(station.country, None);
        assert_eq!(station.value, Some(String::from("Wabush Airport")));
    }
}
