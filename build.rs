use heck::ToPascalCase;
use quick_xml::de::from_reader;
use msc_citypage_sites::{Province, Site, SiteList};
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=siteList.xml");
    println!("cargo:rerun-if-changed=msc_citypage_sites/src/lib.rs");

    let xml_path = "siteList.xml";
    let file = File::open(xml_path).expect("Failed to open siteList.xml");
    let reader = BufReader::new(file);
    let site_list: SiteList = from_reader(reader).expect("Failed to parse siteList.xml");

    let mut sites_by_province: BTreeMap<Province, Vec<&Site>> = BTreeMap::new();
    for site in &site_list.sites {
        sites_by_province
            .entry(site.province)
            .or_default()
            .push(site);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sites_generated.rs");
    let mut output = File::create(&dest_path).expect("Failed to create output file");

    for (province, mut sites) in sites_by_province {
        sites.sort_by(|a, b| a.name_en.cmp(&b.name_en));

        let province_name = format!("{:?}", province);

        writeln!(output, "define_province_sites! {{").unwrap();
        writeln!(output, "Province::{} => [", province_name).unwrap();

        for site in sites {
            let variant_name = sanitize_identifier(&site.name_en);
            writeln!(output, "({}, \"{}\"),", variant_name, site.code).unwrap();
        }

        writeln!(output, "]").unwrap();
        writeln!(output, "}}\n").unwrap();
    }

    let _ = Command::new("rustfmt").arg(&dest_path).status();
}

fn sanitize_identifier(name: &str) -> String {
    let pascal = name.to_pascal_case();

    if pascal.chars().next().is_some_and(|c| c.is_numeric()) {
        format!("Site{}", pascal)
    } else if pascal.is_empty() {
        "Unknown".to_string()
    } else {
        pascal
    }
}
