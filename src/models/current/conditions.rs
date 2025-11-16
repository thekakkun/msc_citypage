use serde::Deserialize;

use crate::models::{
    common::{DateStamp, Format, empty_string_as_none},
    current::{CurrentConditionIcon, Station},
    measurements::{
        humidex::CalculatedHumidex, humidity::RelativeHumidity, pressure::PressureCondition,
        temperature::Temperature, visibility::VisibilityCondition, wind::Wind,
        wind_chill::CalculatedWindChill,
    },
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IconCode {
    #[serde(rename = "$text", default, deserialize_with = "empty_string_as_none")]
    pub value: Option<CurrentConditionIcon>,
    #[serde(rename = "@format", default)]
    pub format: Option<Format>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CurrentConditions {
    pub station: Option<Station>,
    pub date_time: Option<Vec<DateStamp>>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub condition: Option<String>,
    pub icon_code: Option<IconCode>,
    pub temperature: Option<Temperature>,
    pub dewpoint: Option<Temperature>,
    pub wind_chill: Option<CalculatedWindChill>,
    pub humidex: Option<CalculatedHumidex>,
    pub pressure: Option<PressureCondition>,
    pub visibility: Option<VisibilityCondition>,
    pub relative_humidity: Option<RelativeHumidity>,
    pub wind: Option<Wind>,
}
