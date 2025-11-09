use serde::Deserialize;

use crate::models::{
    common::{DateStamp, empty_string_as_none},
    current::Station,
    forecast::IconCode,
    measurements::{
        humidex::CalculatedHumidex, humidity::RelativeHumidity, pressure::PressureCondition,
        temperature::Temperature, visibility::VisibilityCondition, wind::Wind,
        wind_chill::CalculatedWindChill,
    },
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
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
