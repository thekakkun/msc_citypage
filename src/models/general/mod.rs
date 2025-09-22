pub mod date_stamp;

pub use date_stamp::DateStampType;

include!(concat!(env!("OUT_DIR"), "/general.rs"));
