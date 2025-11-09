use serde::Deserialize;

use crate::models::{common::empty_string_as_none, measurements::UnitType};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WindChill {
    #[serde(default)]
    pub text_summary: Option<String>,
    #[serde(rename = "calculated", default)]
    pub calculated: Vec<CalculatedWindChill>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub frostbite: Option<FrostbiteWindChill>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CalculatedWindChill {
    #[serde(rename = "$text", default)]
    pub value: Option<i32>,
    #[serde(rename = "@index", default)]
    pub index: Option<u8>,
    #[serde(
        rename = "@unitType",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub unit_type: Option<UnitType>,
    #[serde(rename = "@class", default, deserialize_with = "empty_string_as_none")]
    pub class: Option<WindChillClass>,
    #[serde(rename = "@qaValue", default)]
    pub qa_value: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum FrostbiteWindChill {
    #[serde(rename = "Risk of frostbite", alias = "Risque d'engelures")]
    RiskOfFrostbite,
    #[serde(
        rename = "Frostbite in minutes",
        alias = "Engelures en quelques minutes"
    )]
    FrostbiteInMinutes,
    #[serde(
        rename = "Hazardous frostbite conditions",
        alias = "Conditions dangereuses d'engelures"
    )]
    HazardousFrostbiteConditions,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum WindChillClass {
    #[serde(rename = "morning", alias = "matin")]
    Morning,
    #[serde(rename = "afternoon", alias = "après-midi")]
    Afternoon,
    #[serde(rename = "evening", alias = "soir")]
    Evening,
    #[serde(rename = "overnight", alias = "nuit")]
    Overnight,
    #[serde(rename = "near", alias = "près de")]
    Near,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_wind_chill() {
        let xml = r#"
            <windChill>
                <textSummary>Wind chill minus 5 this afternoon.</textSummary>
                <calculated unitType="metric" class="afternoon">-5</calculated>
                <frostbite/>
            </windChill>
        "#;
        let wind_chill: WindChill = from_str(xml).unwrap();

        assert_eq!(
            wind_chill.text_summary,
            Some(String::from("Wind chill minus 5 this afternoon."))
        );
        assert_eq!(
            wind_chill.calculated[0],
            CalculatedWindChill {
                value: Some(-5),
                index: None,
                unit_type: Some(UnitType::Metric),
                class: Some(WindChillClass::Afternoon),
                qa_value: None
            }
        );
        assert_eq!(wind_chill.frostbite, None);
    }
}
