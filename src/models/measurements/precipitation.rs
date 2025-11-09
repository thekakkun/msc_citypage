use serde::Deserialize;

use crate::models::{
    common::{deserialize_some_i16, empty_string_as_none},
    measurements::UnitType,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Precipitation {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub text_summary: Option<String>,
    #[serde(rename = "precipType")]
    pub precip_type: Vec<PrecipitationType>,
    #[serde(default)]
    pub accumulation: Vec<Accumulation>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PrecipitationType {
    #[serde(rename = "$text", default, deserialize_with = "empty_string_as_none")]
    pub value: Option<PrecipAbbreviatedCode>,
    #[serde(rename = "@end", default, deserialize_with = "deserialize_some_i16")]
    pub end: Option<i16>,
    #[serde(rename = "@start", default, deserialize_with = "deserialize_some_i16")]
    pub start: Option<i16>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum PrecipAbbreviatedCode {
    #[serde(rename = "rain", alias = "pluie")]
    Rain,
    #[serde(rename = "snow", alias = "neige")]
    Snow,
    #[serde(rename = "freezing rain", alias = "pluie verglaçante")]
    FreezingRain,
    #[serde(rename = "drizzle", alias = "bruine")]
    Drizzle,
    #[serde(rename = "rain and snow", alias = "pluie et neige")]
    RainAndSnow,
    #[serde(
        rename = "rain and freezing rain",
        alias = "pluie et pluie verglaçante"
    )]
    RainAndFreezingRain,
    #[serde(rename = "rain and drizzle", alias = "pluie et bruine")]
    RainAndDrizzle,
    #[serde(
        rename = "snow and freezing rain",
        alias = "neige et pluie verglaçante"
    )]
    SnowAndFreezingRain,
    #[serde(rename = "snow and drizzle", alias = "neige et bruine")]
    SnowAndDrizzle,
    #[serde(
        rename = "freezing rain and drizzle",
        alias = "pluie verglaçante et bruine "
    )]
    FreezingRainAndDrizzle,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Accumulation {
    pub name: AccumulationName,
    pub amount: PrecipitationAmount,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum AccumulationName {
    #[serde(rename = "snow", alias = "neige")]
    Snow,
    #[serde(rename = "rain", alias = "pluie")]
    Rain,
    #[serde(rename = "freezing", alias = "verglaçante")]
    Freezing,
    #[serde(rename = "ice pellets", alias = "grésil")]
    IcePellets,
    #[serde(
        rename = "freezing rain",
        alias = "pluie verglasçante",
        alias = "pluie verglaçante"
    )]
    FreezingRain,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PrecipitationAmount {
    #[serde(rename = "$value")]
    pub value: u16,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<PrecipitationUnit>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@class", default)]
    pub class: Option<PrecipitationClass>,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub period: Option<String>,
    #[serde(rename = "qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum PrecipitationUnit {
    #[serde(rename = "mm")]
    Mm,
    #[serde(rename = "cm")]
    Cm,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PrecipitationClass {
    ExtremeRainfall,
    ExtremeSnowfall,
    ExtremePrecipitation,
    ExtremeSnowOnGround,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_empty_precipitation() {
        let xml = r#"
            <precipitation>
                <textSummary/>
                <precipType start="" end="" />
            </precipitation>
        "#;
        let precipitation: Precipitation = from_str(xml).unwrap();

        assert_eq!(precipitation.text_summary, None);
        assert_eq!(precipitation.precip_type.len(), 1);
        assert_eq!(precipitation.precip_type[0].value, None);
        assert_eq!(precipitation.precip_type[0].start, None);
        assert_eq!(precipitation.precip_type[0].end, None);
        assert_eq!(precipitation.accumulation.len(), 0);
    }

    #[test]
    fn test_deserialize_precipitation() {
        let xml = r#"
            <precipitation>
                <textSummary>Snowfall amount 5 cm.</textSummary>
                <precipType start="37" end="46">snow</precipType>
                <precipType start="46" end="49">rain and snow</precipType>
                <accumulation>
                    <name>rain</name>
                    <amount unitType="metric" units="mm">1</amount>
                </accumulation>
                <accumulation>
                    <name>snow</name>
                    <amount unitType="metric" units="cm">5</amount>
                </accumulation>
            </precipitation>
        "#;
        let precipitation: Precipitation = from_str(xml).unwrap();

        assert_eq!(
            precipitation.text_summary,
            Some(String::from("Snowfall amount 5 cm."))
        );

        let mut precip_type_iter = precipitation.precip_type.iter();
        assert_eq!(
            *precip_type_iter.next().unwrap(),
            PrecipitationType {
                value: Some(PrecipAbbreviatedCode::Snow),
                start: Some(37),
                end: Some(46)
            }
        );
        assert_eq!(
            *precip_type_iter.next().unwrap(),
            PrecipitationType {
                value: Some(PrecipAbbreviatedCode::RainAndSnow),
                start: Some(46),
                end: Some(49)
            }
        );

        let mut accumulation_iter = precipitation.accumulation.iter();
        assert_eq!(
            *accumulation_iter.next().unwrap(),
            Accumulation {
                name: AccumulationName::Rain,
                amount: PrecipitationAmount {
                    value: 1,
                    unit: Some(PrecipitationUnit::Mm),
                    unit_type: Some(UnitType::Metric),
                    class: None,
                    year: None,
                    period: None,
                    qa_value: None
                }
            }
        );
        assert_eq!(
            *accumulation_iter.next().unwrap(),
            Accumulation {
                name: AccumulationName::Snow,
                amount: PrecipitationAmount {
                    value: 5,
                    unit: Some(PrecipitationUnit::Cm),
                    unit_type: Some(UnitType::Metric),
                    class: None,
                    year: None,
                    period: None,
                    qa_value: None
                }
            }
        );
    }
}
