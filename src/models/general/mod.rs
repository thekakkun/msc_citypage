pub mod date_stamp;
pub mod year;

pub use date_stamp::DateStampType;
pub use year::YearType;

include!(concat!(env!("OUT_DIR"), "/general.rs"));
