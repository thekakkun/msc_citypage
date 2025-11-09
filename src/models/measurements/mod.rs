use serde::Deserialize;

pub mod humidex;
pub mod humidity;
pub mod precipitation;
pub mod pressure;
pub mod temperature;
pub mod uv;
pub mod visibility;
pub mod wind;
pub mod wind_chill;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UnitType {
    Imperial,
    Metric,
}
