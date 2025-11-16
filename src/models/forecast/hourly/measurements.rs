use serde::Deserialize;

use crate::models::{
    common::empty_string_as_none,
    forecast::PopUnit,
    measurements::{
        UnitType,
        temperature::TemperatureUnit,
        wind::{WindDirectionValue, WindSpeed},
    },
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TemperatureHourly {
    #[serde(rename = "$text")]
    pub value: f64,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<TemperatureUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LopHourly {
    #[serde(rename = "$text")]
    pub value: u8,
    #[serde(rename = "@category", default)]
    pub category: Option<Category>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<PopUnit>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Category {
    #[serde(rename = "Nil", alias = "Nulle")]
    Nil,
    #[serde(rename = "Low", alias = "Basse")]
    Low,
    #[serde(rename = "Medium", alias = "Moyenne")]
    Medium,
    #[serde(rename = "High", alias = "Élevée")]
    High,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WindChillHourly {
    #[serde(rename = "$text", default)]
    pub value: Option<i32>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct HumidexHourly {
    #[serde(rename = "$text", default)]
    pub value: Option<f32>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WindHourly {
    pub speed: WindSpeed,
    pub direction: WindHourlyDirections,
    pub gust: WindSpeed,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WindHourlyDirections {
    #[serde(rename = "$text", default, deserialize_with = "empty_string_as_none")]
    pub value: Option<WindDirectionValue>,
    #[serde(rename = "@units", default)]
    pub units: Option<UnitsHourly>,
    #[serde(
        rename = "@windDirFull",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub wind_dir_full: Option<WindDirFull>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum UnitsHourly {
    #[serde(rename = "cardinal")]
    Cardinal,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum WindDirFull {
    #[serde(rename = "North", alias = "Nord")]
    North,
    #[serde(rename = "North-northeast", alias = "Nord-nord-est ")]
    NorthNortheast,
    #[serde(rename = "Northeast", alias = "Nord-est")]
    Northeast,
    #[serde(rename = "East-northeast", alias = "Est-nord-est")]
    EastNortheast,
    #[serde(rename = "East", alias = "Est")]
    East,
    #[serde(rename = "East-southeast", alias = "Est-sud-est")]
    EastSoutheast,
    #[serde(rename = "Southeast", alias = "Sud-est")]
    Southeast,
    #[serde(rename = "South-southeast", alias = "Sud-sud-est")]
    SouthSoutheast,
    #[serde(rename = "South", alias = "Sud")]
    South,
    #[serde(rename = "South-southwest", alias = "Sud-sud-ouest")]
    SouthSouthwest,
    #[serde(rename = "Southwest", alias = "Sud-ouest")]
    Southwest,
    #[serde(rename = "West-southwest", alias = "Ouest-sud-ouest")]
    WestSouthwest,
    #[serde(rename = "West", alias = "Ouest")]
    West,
    #[serde(rename = "West-northwest", alias = "Ouest-nord-ouest")]
    WestNorthwest,
    #[serde(rename = "Northwest", alias = "Nord-ouest")]
    Northwest,
    #[serde(rename = "North-northwest", alias = "Nord-nord-ouest")]
    NorthNorthwest,
    #[serde(rename = "Variable direction", alias = "Direction variable")]
    VariableDirection,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct UvHourly {
    pub index: Option<i32>,
}
