use std::fmt::Debug;

use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use quick_xml::events::{BytesStart, BytesText, Event};
use xsd_parser::quick_xml::{
    Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
    WithDeserializer, XmlReader,
};

#[derive(Debug)]
pub struct DateStampType {
    pub datetime: DateTime<Tz>,
    // pub type_: DateStampNameType,
}

impl WithDeserializer for DateStampType
where
    for<'de> DateStampTypeDeserializer: Deserializer<'de, DateStampType>,
{
    type Deserializer = DateStampTypeDeserializer;
}

#[derive(Debug, Default)]
pub struct DateStampTypeDeserializer {
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    hour: Option<u32>,
    minute: Option<u32>,
    timezone: Option<Tz>,
    state: DateStampTypeDeserializerState,
}

#[derive(Debug, Default)]
enum DateStampTypeDeserializerState {
    #[default]
    Init__,
    DateTime,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Unknown__,
}

impl DateStampTypeDeserializer {
    fn handle_datetime(
        &mut self,
        bytes_start: &BytesStart,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for attr_result in bytes_start.attributes() {
            let a = attr_result?;
            if a.key.as_ref() == b"zone" {
                let tz_name = a.decode_and_unescape_value(bytes_start.decoder())?;
                let timezone = match tz_name.as_ref() {
                    "ADT" | "AST" | "HAA" | "HNA" => Some(Tz::Canada__Atlantic),
                    "CDT" | "CST" | "HAC" | "HNC" => Some(Tz::Canada__Central),
                    "EDT" | "EST" | "HAE" | "HNE" => Some(Tz::Canada__Eastern),
                    "MDT" | "MST" | "HAR" | "HNR" => Some(Tz::Canada__Mountain),
                    "NDT" | "NST" | "HAT" | "HNT" => Some(Tz::Canada__Newfoundland),
                    "PDT" | "PST" | "HAP" | "HNP" => Some(Tz::Canada__Pacific),
                    "UTC" => Some(Tz::UTC),
                    _ => None,
                };

                self.timezone = timezone;
            }
        }
        Ok(())
    }

    fn set_year(&mut self, bytes_text: &BytesText) -> Result<(), Box<dyn std::error::Error>> {
        self.year = Some(bytes_text.decode()?.parse()?);
        Ok(())
    }
    fn set_month(&mut self, bytes_text: &BytesText) -> Result<(), Box<dyn std::error::Error>> {
        self.month = Some(bytes_text.decode()?.parse()?);
        Ok(())
    }
    fn set_day(&mut self, bytes_text: &BytesText) -> Result<(), Box<dyn std::error::Error>> {
        self.day = Some(bytes_text.decode()?.parse()?);
        Ok(())
    }
    fn set_hour(&mut self, bytes_text: &BytesText) -> Result<(), Box<dyn std::error::Error>> {
        self.hour = Some(bytes_text.decode()?.parse()?);
        Ok(())
    }
    fn set_minute(&mut self, bytes_text: &BytesText) -> Result<(), Box<dyn std::error::Error>> {
        self.minute = Some(bytes_text.decode()?.parse()?);
        Ok(())
    }
}

impl<'de> Deserializer<'de, DateStampType> for DateStampTypeDeserializer {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, DateStampType>
    where
        R: XmlReader,
    {
        Self::default().next(reader, event)
    }

    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, DateStampType>
    where
        R: XmlReader,
    {
        match event {
            Event::Start(ref bytes_start) => {
                match bytes_start.name().as_ref() {
                    b"dateTime" => {
                        self.state = DateStampTypeDeserializerState::DateTime;
                        self.handle_datetime(&bytes_start);
                    }
                    b"year" => self.state = DateStampTypeDeserializerState::Year,
                    b"month" => self.state = DateStampTypeDeserializerState::Month,
                    b"day" => self.state = DateStampTypeDeserializerState::Day,
                    b"hour" => self.state = DateStampTypeDeserializerState::Hour,
                    b"minute" => self.state = DateStampTypeDeserializerState::Minute,
                    _ => {
                        self.state = DateStampTypeDeserializerState::Unknown__;
                    }
                };
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            }
            Event::End(ref bytes_end) => match bytes_end.name().as_ref() {
                b"dateTime" => {
                    let result = self.finish(reader)?;
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(result),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                b"year" | b"month" | b"day" | b"hour" | b"minute" => {
                    self.state = DateStampTypeDeserializerState::DateTime;
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                _ => {
                    self.state = DateStampTypeDeserializerState::DateTime;
                }
            },
            Event::Empty(_) => {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            }
            Event::Text(ref bytes_text) => {
                let _ = match self.state {
                    DateStampTypeDeserializerState::Year => self.set_year(&bytes_text),
                    DateStampTypeDeserializerState::Month => self.set_month(&bytes_text),
                    DateStampTypeDeserializerState::Day => self.set_day(&bytes_text),
                    DateStampTypeDeserializerState::Hour => self.set_hour(&bytes_text),
                    DateStampTypeDeserializerState::Minute => self.set_minute(&bytes_text),
                    DateStampTypeDeserializerState::Init__
                    | DateStampTypeDeserializerState::DateTime
                    | DateStampTypeDeserializerState::Unknown__ => Ok(()),
                };
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            }
            Event::CData(_)
            | Event::Comment(_)
            | Event::Decl(_)
            | Event::PI(_)
            | Event::DocType(_)
            | Event::GeneralRef(_)
            | Event::Eof => {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            }
        };

        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event: DeserializerEvent::None,
            allow_any: false,
        })
    }

    fn finish<R>(self, _reader: &R) -> Result<DateStampType, xsd_parser::quick_xml::Error>
    where
        R: XmlReader,
    {
        let year = self.year.unwrap();
        let month = self.month.unwrap();
        let day = self.day.unwrap();
        let hour = self.hour.unwrap();
        let min = self.minute.unwrap();
        let timezone = self.timezone.unwrap();

        let datetime = timezone
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .single()
            .unwrap();

        Ok(DateStampType { datetime })
    }
}
