use serde::Deserialize;

use crate::models::{common::empty_string_as_none, measurements::UnitType};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Winds {
    #[serde(default)]
    pub text_summary: Option<String>,
    #[serde(rename = "wind", default)]
    pub wind: Vec<Wind>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Wind {
    pub speed: WindSpeed,
    pub gust: WindSpeed,
    pub direction: WindDirection,
    pub bearing: WindBearing,
    #[serde(rename = "@index", default)]
    pub index: Option<u8>,
    #[serde(rename = "@rank", default, deserialize_with = "empty_string_as_none")]
    pub rank: Option<WindRank>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WindSpeed {
    #[serde(
        rename = "$text",
        default,
        deserialize_with = "deserialize_wind_speed_value"
    )]
    pub value: Option<WindSpeedValue>,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<WindUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindSpeedValue {
    Calm,
    Speed(u16),
}

pub(crate) fn deserialize_wind_speed_value<'de, D>(
    deserializer: D,
) -> Result<Option<WindSpeedValue>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else if s.to_lowercase() == "calm" || s.to_lowercase() == "calme" {
        Ok(Some(WindSpeedValue::Calm))
    } else {
        s.parse::<u16>()
            .map(|v| Some(WindSpeedValue::Speed(v)))
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WindDirection {
    #[serde(rename = "$text", default, deserialize_with = "empty_string_as_none")]
    pub value: Option<WindDirectionValue>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum WindDirectionValue {
    N,
    NNE,
    NE,
    ENE,
    E,
    ESE,
    SE,
    SSE,
    S,
    #[serde(alias = "SSO")]
    SSW,
    #[serde(alias = "SO")]
    SW,
    #[serde(alias = "OSO")]
    WSW,
    #[serde(alias = "O")]
    W,
    #[serde(alias = "ONO")]
    WNW,
    #[serde(alias = "NO")]
    NW,
    #[serde(alias = "NNO")]
    NNW,
    VR,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WindBearing {
    #[serde(
        rename = "$text",
        default,
        deserialize_with = "deserialize_bearing_value"
    )]
    pub value: Option<u16>,
    #[serde(rename = "@units", default)]
    pub unit: Option<WindBearingUnit>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

pub(crate) fn deserialize_bearing_value<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse::<f64>()
            .map(|v| Some(v as u16))
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum WindUnit {
    #[serde(rename = "km/h")]
    KmPerHour,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum WindBearingUnit {
    #[serde(rename = "degrees")]
    Degrees,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum WindRank {
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "minor")]
    Minor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_current_wind() {
        let xml = r#"
            <wind>
                <speed unitType="metric" units="km/h" qaValue="100">calm</speed>
                <gust unitType="metric" units="km/h"/>
                <direction qaValue="100">N</direction>
                <bearing units="degrees" qaValue="100">0.0</bearing>
            </wind>
        "#;
        let wind: Wind = from_str(xml).unwrap();

        assert_eq!(
            wind.speed,
            WindSpeed {
                value: Some(WindSpeedValue::Calm),
                unit: Some(WindUnit::KmPerHour),
                unit_type: Some(UnitType::Metric),
                qa_value: Some(100)
            }
        );
        assert_eq!(
            wind.gust,
            WindSpeed {
                value: None,
                unit: Some(WindUnit::KmPerHour),
                unit_type: Some(UnitType::Metric),
                qa_value: None
            }
        );
        assert_eq!(
            wind.direction,
            WindDirection {
                value: Some(WindDirectionValue::N),
                qa_value: Some(100)
            }
        );
        assert_eq!(
            wind.bearing,
            WindBearing {
                value: Some(0),
                unit: Some(WindBearingUnit::Degrees),
                qa_value: Some(100)
            }
        );
    }

    #[test]
    fn test_deserialize_forecast_wind() {
        let xml = r#"
            <winds>
                <textSummary>Wind southwest 20 km/h becoming light early this evening.</textSummary>
                <wind index="1" rank="major">
                    <speed unitType="metric" units="km/h">20</speed>
                    <gust unitType="metric" units="km/h">00</gust>
                    <direction>SW</direction>
                    <bearing units="degrees">22</bearing>
                </wind>
                <wind index="2" rank="minor">
                    <speed unitType="metric" units="km/h">05</speed>
                    <gust unitType="metric" units="km/h">00</gust>
                    <direction>VR</direction>
                    <bearing units="degrees">99</bearing>
                </wind>
                <wind index="3" rank="major">
                    <speed unitType="metric" units="km/h">10</speed>
                    <gust unitType="metric" units="km/h">00</gust>
                    <direction>NE</direction>
                    <bearing units="degrees">04</bearing>
                </wind>
            </winds>
        "#;
        let winds: Winds = from_str(xml).unwrap();

        assert_eq!(
            winds.text_summary,
            Some(String::from(
                "Wind southwest 20 km/h becoming light early this evening.",
            )),
        );

        let mut wind_iter = winds.wind.iter();
        assert_eq!(
            *wind_iter.next().unwrap(),
            Wind {
                speed: WindSpeed {
                    value: Some(WindSpeedValue::Speed(20)),
                    unit: Some(WindUnit::KmPerHour),
                    unit_type: Some(UnitType::Metric),
                    qa_value: None
                },
                gust: WindSpeed {
                    value: Some(WindSpeedValue::Speed(0)),
                    unit: Some(WindUnit::KmPerHour),
                    unit_type: Some(UnitType::Metric),
                    qa_value: None
                },
                direction: WindDirection {
                    value: Some(WindDirectionValue::SW),
                    qa_value: None
                },
                bearing: WindBearing {
                    value: Some(22),
                    unit: Some(WindBearingUnit::Degrees),
                    qa_value: None
                },
                index: Some(1),
                rank: Some(WindRank::Major)
            }
        );
        assert_eq!(
            *wind_iter.next().unwrap(),
            Wind {
                speed: WindSpeed {
                    value: Some(WindSpeedValue::Speed(5)),
                    unit: Some(WindUnit::KmPerHour),
                    unit_type: Some(UnitType::Metric),
                    qa_value: None
                },
                gust: WindSpeed {
                    value: Some(WindSpeedValue::Speed(0)),
                    unit: Some(WindUnit::KmPerHour),
                    unit_type: Some(UnitType::Metric),
                    qa_value: None
                },
                direction: WindDirection {
                    value: Some(WindDirectionValue::VR),
                    qa_value: None
                },
                bearing: WindBearing {
                    value: Some(99),
                    unit: Some(WindBearingUnit::Degrees),
                    qa_value: None
                },
                index: Some(2),
                rank: Some(WindRank::Minor)
            }
        );
        assert_eq!(
            *wind_iter.next().unwrap(),
            Wind {
                speed: WindSpeed {
                    value: Some(WindSpeedValue::Speed(10)),
                    unit: Some(WindUnit::KmPerHour),
                    unit_type: Some(UnitType::Metric),
                    qa_value: None
                },
                gust: WindSpeed {
                    value: Some(WindSpeedValue::Speed(0)),
                    unit: Some(WindUnit::KmPerHour),
                    unit_type: Some(UnitType::Metric),
                    qa_value: None
                },
                direction: WindDirection {
                    value: Some(WindDirectionValue::NE),
                    qa_value: None
                },
                bearing: WindBearing {
                    value: Some(4),
                    unit: Some(WindBearingUnit::Degrees),
                    qa_value: None
                },
                index: Some(3),
                rank: Some(WindRank::Major)
            }
        )
    }
}
