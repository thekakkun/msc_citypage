use serde::Deserialize;

use super::province::Province;
use crate::models::common::empty_string_as_none;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Location {
    pub continent: String,
    pub country: Country,
    pub province: Province,
    pub name: Name,
    pub region: Region,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Country {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(rename = "@code", default, deserialize_with = "empty_string_as_none")]
    pub code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Name {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(rename = "@code", default, deserialize_with = "empty_string_as_none")]
    pub code: Option<String>,

    #[serde(rename = "@lat", default, deserialize_with = "empty_string_as_none")]
    pub lat: Option<String>,

    #[serde(rename = "@lon", default, deserialize_with = "empty_string_as_none")]
    pub lon: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Region {
    #[serde(rename = "$text", default)]
    pub value: String,

    #[serde(rename = "@code", default, deserialize_with = "empty_string_as_none")]
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_location_en() {
        let xml = r#"
            <location>
                <continent>North America</continent>
                <country code="ca">Canada</country>
                <province code="nb">New Brunswick</province>
                <name code="s0000806" lat="47.61N" lon="65.65W">Bathurst</name>
                <region>Bathurst and Chaleur Region</region>
            </location>
        "#;

        let location: Location = from_str(xml).unwrap();
        assert_eq!(location.continent, String::from("North America"));
        assert_eq!(
            location.country,
            Country {
                code: Some(String::from("ca")),
                value: String::from("Canada")
            }
        );
        assert_eq!(location.province, Province::NewBrunswick);
        assert_eq!(
            location.name,
            Name {
                value: String::from("Bathurst"),
                code: Some(String::from("s0000806")),
                lat: Some(String::from("47.61N")),
                lon: Some(String::from("65.65W")),
            }
        );
        assert_eq!(
            location.region,
            Region {
                value: String::from("Bathurst and Chaleur Region"),
                code: None
            }
        );
    }

    #[test]
    fn test_deserialize_location_fr() {
        let xml = r#"
            <location>
                <continent>Amérique du Nord</continent>
                <country code="ca">Canada</country>
                <province code="nb">Nouveau-Brunswick</province>
                <name code="s0000043" lat="47.36N" lon="68.34O">Edmundston</name>
                <region>Edmundston et comté de Madawaska</region>
            </location>
        "#;

        let location: Location = from_str(xml).unwrap();
        assert_eq!(location.continent, String::from("Amérique du Nord"));
        assert_eq!(
            location.country,
            Country {
                code: Some(String::from("ca")),
                value: String::from("Canada")
            }
        );
        assert_eq!(location.province, Province::NewBrunswick);
        assert_eq!(
            location.name,
            Name {
                value: String::from("Edmundston"),
                code: Some(String::from("s0000043")),
                lat: Some(String::from("47.36N")),
                lon: Some(String::from("68.34O")),
            }
        );
        assert_eq!(
            location.region,
            Region {
                value: String::from("Edmundston et comté de Madawaska"),
                code: None
            }
        );
    }
}
