use serde::Deserialize;

use crate::models::location::province::Province;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename = "siteList")]
pub struct SiteList {
    #[serde(rename = "site", default)]
    pub sites: Vec<Site>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Site {
    #[serde(rename = "@code")]
    pub code: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "nameFr")]
    pub name_fr: String,
    #[serde(rename = "provinceCode")]
    pub province: Province,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_site() {
        let xml = r#"
            <site code="s0000001">
                <nameEn>Athabasca</nameEn>
                <nameFr>Athabasca</nameFr>
                <provinceCode>AB</provinceCode>
            </site>
        "#;

        let site: Site = from_str(xml).unwrap();
        assert_eq!(site.code, "s0000001");
        assert_eq!(site.name_en, "Athabasca");
        assert_eq!(site.name_fr, "Athabasca");
        assert_eq!(site.province, Province::Alberta);
    }
}
