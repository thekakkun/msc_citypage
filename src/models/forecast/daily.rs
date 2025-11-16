use serde::{Deserialize, Deserializer, de};

use crate::models::{
    common::{DateStamp, Format, Period, empty_string_as_none},
    forecast::{ForecastConditionIcon, Frost, RegionalNormals, SnowLevel},
    measurements::{
        humidex::Humidex, humidity::RelativeHumidity, precipitation::Precipitation,
        temperature::Temperatures, uv::Uv, visibility::Visibility, wind::Winds,
        wind_chill::WindChill,
    },
};

fn deserialize_date_stamps<'de, D>(deserializer: D) -> Result<Vec<DateStamp>, D::Error>
where
    D: Deserializer<'de>,
{
    use de::{SeqAccess, Visitor};
    use std::fmt;

    struct DateStampVecVisitor;

    impl<'de> Visitor<'de> for DateStampVecVisitor {
        type Value = Vec<DateStamp>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of DateStamp elements")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            loop {
                match seq.next_element::<DateStamp>() {
                    Ok(Some(elem)) => vec.push(elem),
                    Ok(None) => break,
                    Err(_) => continue, // Skip invalid elements
                }
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(DateStampVecVisitor)
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForecastGroup {
    #[serde(
        rename = "dateTime",
        default,
        deserialize_with = "deserialize_date_stamps"
    )]
    pub date_times: Vec<DateStamp>,
    pub regional_normals: RegionalNormals,
    #[serde(default)]
    pub forecast: Vec<Forecast>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub period: Period,
    pub text_summary: String,
    pub cloud_precip: CloudPrecip,
    pub abbreviated_forecast: AbbreviatedForecast,
    pub temperatures: Temperatures,
    pub winds: Winds,
    pub precipitation: Precipitation,
    #[serde(default)]
    pub snow_level: Option<SnowLevel>,
    #[serde(default)]
    pub wind_chill: Option<WindChill>,
    #[serde(default)]
    pub visibility: Option<Visibility>,
    #[serde(default)]
    pub uv: Option<Uv>,
    #[serde(default)]
    pub relative_humidity: Option<RelativeHumidity>,
    #[serde(default)]
    pub humidex: Option<Humidex>,
    #[serde(default)]
    pub frost: Option<Frost>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CloudPrecip {
    #[serde(default)]
    pub text_summary: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AbbreviatedForecast {
    pub icon_code: IconCode,
    pub pop: Pop,
    #[serde(default)]
    pub text_summary: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IconCode {
    #[serde(rename = "$text", default, deserialize_with = "empty_string_as_none")]
    pub value: Option<ForecastConditionIcon>,
    #[serde(rename = "@format")]
    pub format: Option<Format>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Pop {
    #[serde(rename = "$text", default)]
    pub value: u8,
    #[serde(rename = "@units", default, deserialize_with = "empty_string_as_none")]
    pub unit: Option<PopUnit>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum PopUnit {
    #[serde(rename = "%")]
    Percent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_cloud_precip() {
        let xml = r#"
            <cloudPrecip>
                <textSummary>Cloudy periods with 60 percent chance of flurries.</textSummary>
            </cloudPrecip>
        "#;

        let cloud_precip: CloudPrecip = from_str(xml).unwrap();
        assert_eq!(
            cloud_precip.text_summary,
            Some(String::from(
                "Cloudy periods with 60 percent chance of flurries."
            ))
        )
    }
    #[test]
    fn test_deserialize_pop() {
        let xml = r#"<pop units="%">60</pop>"#;

        let pop: Pop = from_str(xml).unwrap();
        assert_eq!(pop.value, 60);
        assert_eq!(pop.unit, Some(PopUnit::Percent));
    }

    #[test]
    fn test_deserialize_pop_empty() {
        let xml = r#"<pop units="%"/>"#;

        let pop: Pop = from_str(xml).unwrap();
        assert_eq!(pop.value, 0);
        assert_eq!(pop.unit, Some(PopUnit::Percent));
    }

    #[test]
    fn test_deserialize_empty_forecast_group() {
        let xml = r#"
            <forecastGroup>
                <dateTime name="forecastIssue" zone="UTC" UTCOffset="0"/>
                <dateTime name="forecastIssue" zone="" UTCOffset="0"/>
                <regionalNormals>
                <textSummary/>
                <temperature unitType="metric" units="" class="high"/>
                <temperature unitType="metric" units="" class="low"/>
                </regionalNormals>
            </forecastGroup>
        "#;

        let forecast_group: ForecastGroup = from_str(xml).unwrap();

        assert_eq!(forecast_group.date_times, vec![]);

        assert_eq!(forecast_group.regional_normals.text_summary, None);
        assert_eq!(forecast_group.regional_normals.temperatures.len(), 2);

        let high_temp = &forecast_group.regional_normals.temperatures[0];
        assert_eq!(high_temp.value, None);
        assert_eq!(high_temp.unit, None);
        assert_eq!(
            high_temp.unit_type,
            Some(crate::models::measurements::UnitType::Metric)
        );
        assert_eq!(
            high_temp.class,
            Some(crate::models::measurements::temperature::TemperatureClass::High)
        );

        let low_temp = &forecast_group.regional_normals.temperatures[1];
        assert_eq!(low_temp.value, None);
        assert_eq!(low_temp.unit, None);
        assert_eq!(
            low_temp.unit_type,
            Some(crate::models::measurements::UnitType::Metric)
        );
        assert_eq!(
            low_temp.class,
            Some(crate::models::measurements::temperature::TemperatureClass::Low)
        );

        assert_eq!(forecast_group.forecast, vec![]);
    }
}
