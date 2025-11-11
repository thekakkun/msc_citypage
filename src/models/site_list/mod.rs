pub use msc_citypage_sites::{Site, SiteList};

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    use msc_citypage_sites::Province;

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
