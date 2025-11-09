use crate::models::common::{DateStamp, empty_string_as_none, serde_helpers::deserialize_some_url};

use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Warnings {
    pub event: Option<Vec<WarningEvent>>,

    #[serde(rename = "@url", deserialize_with = "deserialize_some_url", default)]
    pub url: Option<Url>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WarningEvent {
    pub date_time: Option<Vec<DateStamp>>,

    #[serde(rename = "@type")]
    pub warning_type: Option<WarningType>,

    #[serde(
        rename = "@description",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "@priority")]
    pub priority: Option<WarningPriority>,

    #[serde(
        rename = "@expiryTime",
        default,
        deserialize_with = "empty_string_as_none"
    )]
    pub expiry_time: Option<String>,

    #[serde(rename = "@url", deserialize_with = "deserialize_some_url")]
    pub url: Option<Url>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WarningType {
    Advisory,
    Warning,
    Watch,
    Ended,
    Statement,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WarningPriority {
    Urgent,
    High,
    Medium,
    Low,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_warnings_en() {
        let xml = r#"<warnings>
                <event type="advisory" priority="low" description="FROST ADVISORY" expiryTime="20251027063224" url="https://weather.gc.ca/warnings/report_e.html?onrm18#123006231930311153202510250503">
                    <dateTime name="eventIssue" zone="UTC" UTCOffset="0">
                        <year>2025</year>
                        <month name="October">10</month>
                        <day name="Sunday">26</day>
                        <hour>14</hour>
                        <minute>32</minute>
                        <timeStamp>20251026143224</timeStamp>
                        <textSummary>Sunday October 26, 2025 at 14:32 UTC</textSummary>
                    </dateTime>
                    <dateTime name="eventIssue" zone="EDT" UTCOffset="-4">
                        <year>2025</year>
                        <month name="October">10</month>
                        <day name="Sunday">26</day>
                        <hour>10</hour>
                        <minute>32</minute>
                        <timeStamp>20251026103224</timeStamp>
                        <textSummary>Sunday October 26, 2025 at 10:32 EDT</textSummary>
                    </dateTime>
                </event>
            </warnings>"#;
        let warnings: Warnings = from_str(xml).unwrap();

        assert_eq!(warnings.url, None);

        let events = warnings.event.unwrap();
        let mut event_iter = events.iter();
        let event = event_iter.next().unwrap();

        assert_eq!(event.warning_type, Some(WarningType::Advisory));
        assert_eq!(event.description, Some(String::from("FROST ADVISORY")));
        assert_eq!(event.priority, Some(WarningPriority::Low));
        assert_eq!(event.expiry_time, Some(String::from("20251027063224")));
        assert_eq!(
            event.url,
            Url::parse(
                "https://weather.gc.ca/warnings/report_e.html?onrm18#123006231930311153202510250503"
            )
            .ok()
        );
    }
}
