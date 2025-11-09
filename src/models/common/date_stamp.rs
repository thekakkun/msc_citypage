use std::str::FromStr;

use chrono::{DateTime, Month, TimeZone, Weekday};
use chrono_tz::Tz;
use serde::{Deserialize, Deserializer, de};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DateStamp {
    pub year: i32,
    #[serde(deserialize_with = "deserialize_month")]
    pub month: Month,
    pub day: Day,
    pub hour: u32,
    pub minute: u32,
    pub time_stamp: String,
    pub text_summary: String,
    #[serde(rename = "@name", default)]
    pub name: Option<DateStampName>,
    #[serde(rename = "@zone", default, deserialize_with = "deserialize_zone")]
    pub zone: Option<Tz>,
    #[serde(rename = "@UTCOffset")]
    pub utc_offset: Option<f32>,
}

impl TryFrom<DateStamp> for DateTime<Tz> {
    type Error = String;

    fn try_from(value: DateStamp) -> Result<Self, Self::Error> {
        let tz = value.zone.ok_or("Missing timezone")?;
        let datetime = tz
            .with_ymd_and_hms(
                value.year,
                value.month.number_from_month(),
                value.day.value,
                value.hour,
                value.minute,
                0,
            )
            .single()
            .unwrap();

        Ok(datetime)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum DateStampName {
    #[serde(rename = "observation")]
    Observation,
    #[serde(rename = "xmlCreation")]
    XmlCreation,
    #[serde(rename = "forecastIssue")]
    ForecastIssue,
    #[serde(rename = "eventIssue")]
    EventIssue,
    #[serde(rename = "sunrise")]
    Sunrise,
    #[serde(rename = "sunset")]
    Sunset,
    #[serde(rename = "moonrise")]
    Moonrise,
    #[serde(rename = "moonset")]
    Moonset,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Day {
    #[serde(rename = "$text")]
    pub value: u32,

    #[serde(rename = "@name", deserialize_with = "deserialize_weekday")]
    pub weekday: Weekday,
}

fn deserialize_month<'de, D>(deserializer: D) -> Result<Month, D::Error>
where
    D: Deserializer<'de>,
{
    let m = u8::deserialize(deserializer)?;

    match Month::try_from(m) {
        Ok(month) => Ok(month),
        Err(_) => Err(de::Error::custom(format!("Invalid month value: {}", m))),
    }
}

fn deserialize_zone<'de, D>(deserializer: D) -> Result<Option<Tz>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() {
        return Ok(None);
    }

    let tz = match s.as_str() {
        "ADT" | "AST" | "HAA" | "HNA" => Some(Tz::Canada__Atlantic),
        "CDT" | "CST" | "HAC" | "HNC" => Some(Tz::Canada__Central),
        "EDT" | "EST" | "HAE" | "HNE" => Some(Tz::Canada__Eastern),
        "MDT" | "MST" | "HAR" | "HNR" => Some(Tz::Canada__Mountain),
        "NDT" | "NST" | "HAT" | "HNT" => Some(Tz::Canada__Newfoundland),
        "PDT" | "PST" | "HAP" | "HNP" => Some(Tz::Canada__Pacific),
        "UTC" => Some(Tz::UTC),
        _ => {
            return Err(de::Error::custom(format!("Invalid timezone name: {}", s)));
        }
    };

    Ok(tz)
}

fn deserialize_weekday<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if let Ok(weekday) = Weekday::from_str(&s) {
        return Ok(weekday);
    }
    let weekday = match s.to_lowercase().as_str() {
        "lundi" => Weekday::Mon,
        "mardi" => Weekday::Tue,
        "mercredi" => Weekday::Wed,
        "jeudi" => Weekday::Thu,
        "vendredi" => Weekday::Fri,
        "samedi" => Weekday::Sat,
        "dimanche" => Weekday::Sun,
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Invalid weekday name: {}",
                s
            )));
        }
    };

    Ok(weekday)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::UTC;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_date_stamp_en() {
        let xml = r#"
            <dateTime name="xmlCreation" zone="UTC" UTCOffset="0">
                <year>2025</year>
                <month name="October">10</month>
                <day name="Monday">13</day>
                <hour>14</hour>
                <minute>01</minute>
                <timeStamp>20251013140104</timeStamp>
                <textSummary>Monday October 13, 2025 at 14:01 UTC</textSummary>
            </dateTime>
        "#;

        let date_stamp: DateStamp = from_str(xml).unwrap();
        assert_eq!(date_stamp.year, 2025);
        assert_eq!(date_stamp.month, Month::October);
        assert_eq!(
            date_stamp.day,
            Day {
                value: 13,
                weekday: Weekday::Mon
            }
        );
        assert_eq!(date_stamp.hour, 14);
        assert_eq!(date_stamp.minute, 1);
        assert_eq!(date_stamp.time_stamp, String::from("20251013140104"));
        assert_eq!(
            date_stamp.text_summary,
            String::from("Monday October 13, 2025 at 14:01 UTC")
        );
        assert_eq!(date_stamp.name, Some(DateStampName::XmlCreation));
        assert_eq!(date_stamp.zone, Some(UTC));
        assert_eq!(date_stamp.utc_offset, Some(0.0));
    }

    #[test]
    fn test_deserialize_date_stamp_fr() {
        let xml = r#"
            <dateTime name="observation" zone="HAA" UTCOffset="-3">
                <year>2025</year>
                <month name="octobre">10</month>
                <day name="lundi">13</day>
                <hour>11</hour>
                <minute>00</minute>
                <timeStamp>20251013110000</timeStamp>
                <textSummary>13 octobre 2025 11h00 HAA</textSummary>
            </dateTime>
        "#;

        let date_stamp: DateStamp = from_str(xml).unwrap();
        assert_eq!(date_stamp.year, 2025);
        assert_eq!(date_stamp.month, Month::October);
        assert_eq!(
            date_stamp.day,
            Day {
                value: 13,
                weekday: Weekday::Mon
            }
        );
        assert_eq!(date_stamp.hour, 11);
        assert_eq!(date_stamp.minute, 0);
        assert_eq!(date_stamp.time_stamp, String::from("20251013110000"));
        assert_eq!(
            date_stamp.text_summary,
            String::from("13 octobre 2025 11h00 HAA")
        );
        assert_eq!(date_stamp.zone, Some(Tz::Canada__Atlantic));
        assert_eq!(date_stamp.utc_offset, Some(-3.0));
    }

    #[test]
    fn test_deserialize_date_stamp_no_zone() {
        let xml = r#"
            <dateTime name="xmlCreation">
                <year>2025</year>
                <month name="October">10</month>
                <day name="Monday">13</day>
                <hour>14</hour>
                <minute>01</minute>
                <timeStamp>20251013140104</timeStamp>
                <textSummary>Monday October 13, 2025 at 14:01 UTC</textSummary>
            </dateTime>
        "#;

        let date_stamp: DateStamp = from_str(xml).unwrap();
        assert_eq!(date_stamp.zone, None);
        assert_eq!(date_stamp.utc_offset, None);
    }
}
