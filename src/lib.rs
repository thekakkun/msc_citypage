#[allow(
    dead_code,
    unused_mut,
    unused_variables,
    clippy::never_loop,
    clippy::single_match,
    clippy::redundant_field_names
)]
#[rustfmt::skip]
mod schema;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use xsd_parser::quick_xml::{DeserializeSync, IoReader, XmlReader};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn parse_weather() {
        let input_file =
            std::fs::File::open("20250830T193948.467Z_MSC_CitypageWeather_s0000024_en.xml")
                .unwrap();
        let reader = BufReader::new(input_file);
        let mut reader = IoReader::new(reader).with_error_info();
        let mut doc = schema::SiteData::deserialize(&mut reader).unwrap();

        print!("Created structure = {:#?}\n\n", doc);
    }
}
