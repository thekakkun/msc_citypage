use msc_citypage::models::catalog::SiteList;
use quick_xml::de::from_reader;
use std::{fs::File, io::BufReader};

/// An example of deserializing a SiteData XML.
fn main() {
    let input_file = File::open("schema_files/siteList.xml").unwrap();
    let reader = BufReader::new(input_file);

    let site_data: SiteList = from_reader(reader).unwrap();

    println!("{:#?}", site_data);
}
