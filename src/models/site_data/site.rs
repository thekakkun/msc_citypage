use serde::Deserialize;
use url::Url;

use crate::models::{
    common::{DateStamp, serde_helpers::deserialize_url},
    current::CurrentConditions,
    forecast::{ForecastGroup, HourlyForecastGroup, RiseSet},
    location::Location,
    warnings::Warnings,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteData {
    #[serde(deserialize_with = "deserialize_url")]
    pub license: Url,

    #[serde(rename = "dateTime")]
    pub date_times: (DateStamp, DateStamp),

    pub location: Location,
    pub warnings: Warnings,

    #[serde(rename = "currentConditions")]
    pub current_conditions: CurrentConditions,

    #[serde(rename = "forecastGroup")]
    pub forecast_group: ForecastGroup,

    #[serde(rename = "hourlyForecastGroup")]
    pub hourly_forecast_group: HourlyForecastGroup,

    #[serde(rename = "riseSet")]
    pub rise_set: RiseSet,
}
