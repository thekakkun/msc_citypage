mod measurements;

pub use measurements::{
    HumidexHourly, IconCodeHourly, LopHourly, TemperatureHourly, UvHourly, WindChillHourly,
    WindHourly,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

use crate::models::common::DateStamp;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecastGroup {
    #[serde(rename = "dateTime", default)]
    pub date_time: Vec<DateStamp>,
    #[serde(default)]
    pub hourly_forecast: Vec<HourlyForecast>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecast {
    pub condition: String,
    pub icon_code: IconCodeHourly,
    pub temperature: TemperatureHourly,
    pub lop: LopHourly,
    pub wind_chill: WindChillHourly,
    pub humidex: HumidexHourly,
    pub wind: WindHourly,
    #[serde(default)]
    pub uv: Option<UvHourly>,
    #[serde(
        rename = "dateTimeUTC",
        default,
        deserialize_with = "deserialize_some_date_time"
    )]
    pub date_time_utc: Option<DateTime<Utc>>,
}

pub(crate) fn deserialize_some_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() {
        return Ok(None);
    }

    NaiveDateTime::parse_from_str(&s, "%Y%m%d%H%M")
        .map(|dt| Some(dt.and_utc()))
        .map_err(serde::de::Error::custom)
}
