#[allow(
    dead_code,
    unused_mut,
    unused_variables,
    clippy::never_loop,
    clippy::single_match,
    clippy::redundant_field_names
)]
#[rustfmt::skip]
pub mod models;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Read};

    use encoding_rs::WINDOWS_1252;
    use xsd_parser::quick_xml::{DeserializeSync, IoReader, XmlReader};

    use crate::models::generated_schema::{SiteData, SiteList};

    #[test]
    fn parse_site_data() {
        let input_file =
            std::fs::File::open("20250904T020028.758Z_MSC_CitypageWeather_s0000168_en.xml")
                .unwrap();
        let reader = BufReader::new(input_file);
        let mut reader = IoReader::new(reader).with_error_info();
        let mut doc = SiteData::deserialize(&mut reader).unwrap();

        print!("created structure = {:#?}\n\n", doc);
    }

    #[test]
    fn parse_site_list() {
        let input_file = std::fs::File::open("siteList.xml").unwrap();
        let mut buf = Vec::new();
        BufReader::new(input_file).read_to_end(&mut buf).unwrap();

        let (cow, _, had_errors) = WINDOWS_1252.decode(&buf);
        let mut reader = IoReader::new(cow.as_bytes()).with_error_info();

        let doc = SiteList::deserialize(&mut reader).unwrap();

        print!("created structure = {:#?}\n\n", doc);
    }
}
