use serde::Deserialize;

use crate::models::common::empty_string_as_none;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Uv {
    #[serde(default)]
    pub index: Option<i32>,
    #[serde(default)]
    pub text_summary: Option<String>,
    #[serde(
        rename = "@category",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub category: Option<UvCategory>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum UvCategory {
    #[serde(rename = "low", alias = "bas")]
    Low,
    #[serde(rename = "moderate", alias = "modéré")]
    Moderate,
    #[serde(rename = "high", alias = "élevé")]
    High,
    #[serde(rename = "very high", alias = "très élevé")]
    VeryHigh,
    #[serde(rename = "extreme", alias = "extrême")]
    Extreme,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_uv_en() {
        let xml = r#"
            <uv category="low">
                <index>2</index>
                <textSummary>UV index 2 or low.</textSummary>
            </uv>
        "#;
        let uv: Uv = from_str(xml).unwrap();

        assert_eq!(uv.category, Some(UvCategory::Low));
        assert_eq!(uv.index, Some(2));
        assert_eq!(uv.text_summary, Some(String::from("UV index 2 or low.")));
    }

    #[test]
    fn test_deserialize_uv_fr() {
        let xml = r#"
            <uv category="bas">
                <index>1</index>
                <textSummary>Indice UV de 1 ou bas.</textSummary>
            </uv>
        "#;
        let uv: Uv = from_str(xml).unwrap();

        assert_eq!(uv.category, Some(UvCategory::Low));
        assert_eq!(uv.index, Some(1));
        assert_eq!(
            uv.text_summary,
            Some(String::from("Indice UV de 1 ou bas."))
        );
    }
}
