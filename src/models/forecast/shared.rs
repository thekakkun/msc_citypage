use serde::Deserialize;

use crate::models::{
    common::{DateStamp, empty_string_as_none},
    measurements::temperature::Temperature,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiseSet {
    #[serde(rename = "dateTime", default)]
    pub date_times: Vec<DateStamp>,
    #[serde(default)]
    pub disclaimer: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegionalNormals {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub text_summary: Option<String>,

    #[serde(rename = "temperature", default)]
    pub temperatures: Vec<Temperature>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SnowLevel {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub text_summary: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Frost {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub text_summary: Option<String>,
}
