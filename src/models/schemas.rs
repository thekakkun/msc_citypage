use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use serde::{Deserialize, de::Error as DeError};
use xsd_parser::quick_xml::DeserializeBytesFromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XsDateTime(pub DateTime<Utc>);

impl FromStr for XsDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S")
            .map(|datetime| Self(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc)))
    }
}

impl<'de> Deserialize<'de> for XsDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s.parse().map_err(|_| {
            DeError::custom("DateTime. Invalid value!")
        })?))
    }
}
impl DeserializeBytesFromStr for XsDateTime {}
