use serde::Deserialize;

use crate::models::common::DayName;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Period {
    #[serde(rename = "$text")]
    pub value: DayName,
    #[serde(rename = "@textForecastName", default)]
    pub text_forecast_name: Option<DayName>,
}
