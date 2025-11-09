use std::{fs::File, io::BufReader};

use msc_citypage::SiteData;
use quick_xml::de::from_reader;

/// An example of deserializing a SiteData XML.
fn main() {
    let input_file = File::open("examples/data.xml").unwrap();
    let reader = BufReader::new(input_file);

    match from_reader::<_, SiteData>(reader) {
        Ok(site_data) => println!("{:#?}", site_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
