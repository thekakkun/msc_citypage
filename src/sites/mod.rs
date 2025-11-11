use crate::models::location::Province;

#[macro_use]
mod macros;

pub trait SiteInfo {
    fn code(&self) -> String;
    fn province(&self) -> Province;
}

include!(concat!(env!("OUT_DIR"), "/sites_generated.rs"));
