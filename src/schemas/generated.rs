use crate::schemas::{DateTimeUtcType, TimeStampType};
use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{
        DeserializeBytes, DeserializeReader, Error, ErrorKind, RawByteStr, WithDeserializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
#[derive(Debug)]
pub struct AbbreviatedForecastType {
    pub icon_code: IconCodeType,
    pub pop: PopType,
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for AbbreviatedForecastType {
    type Deserializer = quick_xml_deserialize::AbbreviatedForecastTypeDeserializer;
}
#[derive(Debug)]
pub struct AccumulationType {
    pub name: ValidAccumulationNamesType,
    pub amount: PrecipType,
}
impl WithDeserializer for AccumulationType {
    type Deserializer = quick_xml_deserialize::AccumulationTypeDeserializer;
}
#[derive(Debug)]
pub struct CalculatedHumidexType {
    pub unit_type: Option<ValidUnitTypesType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidHumidexType,
}
impl WithDeserializer for CalculatedHumidexType {
    type Deserializer = quick_xml_deserialize::CalculatedHumidexTypeDeserializer;
}
#[derive(Debug)]
pub struct CalculatedWindChillType {
    pub index: Option<::core::primitive::i32>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub class: Option<ValidWindChillClassType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidWindChillsType,
}
impl WithDeserializer for CalculatedWindChillType {
    type Deserializer = quick_xml_deserialize::CalculatedWindChillTypeDeserializer;
}
#[derive(Debug)]
pub enum CategoryType {
    Nil,
    Low,
    Medium,
    High,
    Nulle,
    Basse,
    Moyenne,
    Élevée,
}
impl DeserializeBytes for CategoryType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"Nil" => Ok(Self::Nil),
            b"Low" => Ok(Self::Low),
            b"Medium" => Ok(Self::Medium),
            b"High" => Ok(Self::High),
            b"Nulle" => Ok(Self::Nulle),
            b"Basse" => Ok(Self::Basse),
            b"Moyenne" => Ok(Self::Moyenne),
            b"\xC3\x89lev\xC3\xA9e" => Ok(Self::Élevée),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct CloudPrecipType {
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for CloudPrecipType {
    type Deserializer = quick_xml_deserialize::CloudPrecipTypeDeserializer;
}
pub type ConditionType = ::std::string::String;
pub type ContinentType = ::std::string::String;
#[derive(Debug)]
pub struct CountryType {
    pub code: Option<::std::string::String>,
    pub content: ::std::string::String,
}
impl WithDeserializer for CountryType {
    type Deserializer = quick_xml_deserialize::CountryTypeDeserializer;
}
#[derive(Debug)]
pub struct CurrentConditionsType {
    pub station: Option<StationType>,
    pub date_time: Vec<DateStampType>,
    pub condition: Option<::std::string::String>,
    pub icon_code: Option<IconCodeType>,
    pub temperature: Option<TemperatureType>,
    pub dewpoint: Option<TemperatureType>,
    pub wind_chill: Option<CalculatedWindChillType>,
    pub humidex: Option<CalculatedHumidexType>,
    pub pressure: Option<PressureTypeCondType>,
    pub visibility: Option<VisibilityTypeCondType>,
    pub relative_humidity: Option<RelativeHumidityType>,
    pub wind: Option<WindType>,
}
impl WithDeserializer for CurrentConditionsType {
    type Deserializer = quick_xml_deserialize::CurrentConditionsTypeDeserializer;
}
#[derive(Debug)]
pub enum DateStampNameType {
    Observation,
    XmlCreation,
    ForecastIssue,
    EventIssue,
    Sunrise,
    Sunset,
    Moonrise,
    Moonset,
}
impl DeserializeBytes for DateStampNameType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"observation" => Ok(Self::Observation),
            b"xmlCreation" => Ok(Self::XmlCreation),
            b"forecastIssue" => Ok(Self::ForecastIssue),
            b"eventIssue" => Ok(Self::EventIssue),
            b"sunrise" => Ok(Self::Sunrise),
            b"sunset" => Ok(Self::Sunset),
            b"moonrise" => Ok(Self::Moonrise),
            b"moonset" => Ok(Self::Moonset),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct DateStampType {
    pub name: Option<DateStampNameType>,
    pub zone: Option<ValidTimeZonesType>,
    pub utc_offset: Option<::core::primitive::f64>,
    pub year: ::std::string::String,
    pub month: MonthType,
    pub day: DayType,
    pub hour: HourType,
    pub minute: MinuteType,
    pub time_stamp: TimeStampType,
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for DateStampType {
    type Deserializer = quick_xml_deserialize::DateStampTypeDeserializer;
}
#[derive(Debug)]
pub struct DayType {
    pub name: ValidDayNamesType,
    pub content: ::core::primitive::usize,
}
impl WithDeserializer for DayType {
    type Deserializer = quick_xml_deserialize::DayTypeDeserializer;
}
#[derive(Debug)]
pub struct ForecastGroupTypeFullType {
    pub date_time: [DateStampType; 2usize],
    pub regional_normals: RegionalNormalsType,
    pub forecast: Vec<ForecastTypeFullType>,
}
impl WithDeserializer for ForecastGroupTypeFullType {
    type Deserializer = quick_xml_deserialize::ForecastGroupTypeFullTypeDeserializer;
}
#[derive(Debug)]
pub struct ForecastTypeFullType {
    pub period: PeriodType,
    pub text_summary: ::std::string::String,
    pub cloud_precip: CloudPrecipType,
    pub abbreviated_forecast: AbbreviatedForecastType,
    pub temperatures: TemperaturesType,
    pub winds: WindsType,
    pub precipitation: PrecipTypeForecastType,
    pub snow_level: Option<SnowLevelType>,
    pub wind_chill: Option<WindChillType>,
    pub visibility: Option<VisibilityTypeForecastType>,
    pub uv: Option<UvType>,
    pub relative_humidity: Option<RelativeHumidityType>,
    pub humidex: Option<HumidexType>,
    pub frost: Option<FrostType>,
}
impl WithDeserializer for ForecastTypeFullType {
    type Deserializer = quick_xml_deserialize::ForecastTypeFullTypeDeserializer;
}
#[derive(Debug)]
pub struct FrostType {
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for FrostType {
    type Deserializer = quick_xml_deserialize::FrostTypeDeserializer;
}
#[derive(Debug)]
pub enum FrostbiteWindChillType {
    RiskOfFrostbite,
    FrostbiteInMinutes,
    HazardousFrostbiteConditions,
    RisqueDEngelures,
    EngeluresEnQuelquesMinutes,
    ConditionsDangereusesDEngelures,
}
impl DeserializeBytes for FrostbiteWindChillType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"Risk of frostbite" => Ok(Self::RiskOfFrostbite),
            b"Frostbite in minutes" => Ok(Self::FrostbiteInMinutes),
            b"Hazardous frostbite conditions" => Ok(Self::HazardousFrostbiteConditions),
            b"Risque d'engelures" => Ok(Self::RisqueDEngelures),
            b"Engelures en quelques minutes" => Ok(Self::EngeluresEnQuelquesMinutes),
            b"Conditions dangereuses d'engelures" => Ok(Self::ConditionsDangereusesDEngelures),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct HourType {
    pub content: ValidHoursType,
}
impl WithDeserializer for HourType {
    type Deserializer = quick_xml_deserialize::HourTypeDeserializer;
}
#[derive(Debug)]
pub struct HourlyForecastGroupTypeFullType {
    pub date_time: Vec<DateStampType>,
    pub hourly_forecast: Vec<HourlyForecastTypeFullType>,
}
impl WithDeserializer for HourlyForecastGroupTypeFullType {
    type Deserializer = quick_xml_deserialize::HourlyForecastGroupTypeFullTypeDeserializer;
}
#[derive(Debug)]
pub struct HourlyForecastTypeFullType {
    pub date_time_utc: Option<DateTimeUtcType>,
    pub condition: xs::AnyType,
    pub icon_code: IconCodeHourlyType,
    pub temperature: TemperatureHourlyType,
    pub lop: LopHourlyType,
    pub wind_chill: WindChillHourlyType,
    pub humidex: HumidexHourlyType,
    pub wind: WindHourlyType,
    pub uv: Option<UvHourlyType>,
}
impl WithDeserializer for HourlyForecastTypeFullType {
    type Deserializer = quick_xml_deserialize::HourlyForecastTypeFullTypeDeserializer;
}
#[derive(Debug)]
pub struct HumidexHourlyType {
    pub unit_type: Option<ValidUnitTypesType>,
    pub content: ValidHumidexType,
}
impl WithDeserializer for HumidexHourlyType {
    type Deserializer = quick_xml_deserialize::HumidexHourlyTypeDeserializer;
}
#[derive(Debug)]
pub struct HumidexType {
    pub text_summary: Option<::std::string::String>,
    pub calculated: Vec<CalculatedHumidexType>,
}
impl WithDeserializer for HumidexType {
    type Deserializer = quick_xml_deserialize::HumidexTypeDeserializer;
}
#[derive(Debug)]
pub struct IconCodeHourlyType {
    pub format: Option<ValidFormatType>,
    pub content: ::std::string::String,
}
impl WithDeserializer for IconCodeHourlyType {
    type Deserializer = quick_xml_deserialize::IconCodeHourlyTypeDeserializer;
}
#[derive(Debug)]
pub struct IconCodeType {
    pub format: ValidIconCodesUnitsType,
    pub content: ValidIconCodesType,
}
impl WithDeserializer for IconCodeType {
    type Deserializer = quick_xml_deserialize::IconCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct LocationType {
    pub continent: ::std::string::String,
    pub country: CountryType,
    pub province: ProvinceType,
    pub name: NameType,
    pub region: RegionType,
}
impl WithDeserializer for LocationType {
    type Deserializer = quick_xml_deserialize::LocationTypeDeserializer;
}
#[derive(Debug)]
pub struct LopHourlyType {
    pub category: Option<CategoryType>,
    pub units: Option<ValidPopUnitsType>,
    pub content: ValidPopsType,
}
impl WithDeserializer for LopHourlyType {
    type Deserializer = quick_xml_deserialize::LopHourlyTypeDeserializer;
}
#[derive(Debug)]
pub enum MinuteType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for MinuteType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub struct MonthType {
    pub name: ValidMonthNamesType,
    pub content: ::core::primitive::i32,
}
impl WithDeserializer for MonthType {
    type Deserializer = quick_xml_deserialize::MonthTypeDeserializer;
}
#[derive(Debug)]
pub struct NameType {
    pub code: Option<::std::string::String>,
    pub lat: Option<::std::string::String>,
    pub lon: Option<::std::string::String>,
    pub content: ::std::string::String,
}
impl WithDeserializer for NameType {
    type Deserializer = quick_xml_deserialize::NameTypeDeserializer;
}
#[derive(Debug)]
pub enum PeriodRangeType {
    String(::std::string::String),
    None,
}
impl DeserializeBytes for PeriodRangeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::String(::std::string::String::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub struct PeriodType {
    pub text_forecast_name: Option<ValidDayNamesType>,
    pub content: ValidDayNamesType,
}
impl WithDeserializer for PeriodType {
    type Deserializer = quick_xml_deserialize::PeriodTypeDeserializer;
}
#[derive(Debug)]
pub struct PopType {
    pub units: Option<ValidPopUnitsType>,
    pub content: ValidPopsType,
}
impl WithDeserializer for PopType {
    type Deserializer = quick_xml_deserialize::PopTypeDeserializer;
}
#[derive(Debug)]
pub struct PrecipSubTypeForecastType {
    pub end: Option<ValidPrecipSubTypeForecastHoursType>,
    pub start: Option<ValidPrecipSubTypeForecastHoursType>,
    pub content: ValidPrecipAbbreviatedCodesType,
}
impl WithDeserializer for PrecipSubTypeForecastType {
    type Deserializer = quick_xml_deserialize::PrecipSubTypeForecastTypeDeserializer;
}
#[derive(Debug)]
pub struct PrecipType {
    pub units: Option<ValidPrecipUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub class: Option<ValidPrecipClassesType>,
    pub year: Option<::std::string::String>,
    pub period: Option<PeriodRangeType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidPrecipsType,
}
impl WithDeserializer for PrecipType {
    type Deserializer = quick_xml_deserialize::PrecipTypeDeserializer;
}
#[derive(Debug)]
pub struct PrecipTypeForecastType {
    pub text_summary: ::std::string::String,
    pub precip_type: Vec<PrecipSubTypeForecastType>,
    pub accumulation: Vec<AccumulationType>,
}
impl WithDeserializer for PrecipTypeForecastType {
    type Deserializer = quick_xml_deserialize::PrecipTypeForecastTypeDeserializer;
}
#[derive(Debug)]
pub struct PressureTypeCondType {
    pub units: Option<ValidPressureUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub change: Option<ValidPressuresType>,
    pub tendency: Option<ValidPressureTendenciesType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidPressuresType,
}
impl WithDeserializer for PressureTypeCondType {
    type Deserializer = quick_xml_deserialize::PressureTypeCondTypeDeserializer;
}
#[derive(Debug)]
pub struct PressureTypeForecastType {
    pub units: Option<ValidPressureUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub content: ValidPressuresType,
}
impl WithDeserializer for PressureTypeForecastType {
    type Deserializer = quick_xml_deserialize::PressureTypeForecastTypeDeserializer;
}
#[derive(Debug)]
pub struct PressuresType {
    pub text_summary: ::std::string::String,
    pub pressure: PressureTypeForecastType,
}
impl WithDeserializer for PressuresType {
    type Deserializer = quick_xml_deserialize::PressuresTypeDeserializer;
}
#[derive(Debug)]
pub struct ProvinceType {
    pub code: Option<::std::string::String>,
    pub content: ::std::string::String,
}
impl WithDeserializer for ProvinceType {
    type Deserializer = quick_xml_deserialize::ProvinceTypeDeserializer;
}
#[derive(Debug)]
pub enum QaValueType {
    _10,
    _100,
}
impl DeserializeBytes for QaValueType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"10" => Ok(Self::_10),
            b"100" => Ok(Self::_100),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct RegionType {
    pub code: Option<::std::string::String>,
    pub content: ::std::string::String,
}
impl WithDeserializer for RegionType {
    type Deserializer = quick_xml_deserialize::RegionTypeDeserializer;
}
#[derive(Debug)]
pub struct RegionalNormalsType {
    pub text_summary: Option<::std::string::String>,
    pub temperature: Vec<TemperatureType>,
}
impl WithDeserializer for RegionalNormalsType {
    type Deserializer = quick_xml_deserialize::RegionalNormalsTypeDeserializer;
}
#[derive(Debug)]
pub struct RelativeHumidityType {
    pub units: Option<ValidRelativeHumidityUnitsType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidRelativeHumiditiesType,
}
impl WithDeserializer for RelativeHumidityType {
    type Deserializer = quick_xml_deserialize::RelativeHumidityTypeDeserializer;
}
#[derive(Debug)]
pub struct RiseSetElementType {
    pub time: TimeType,
}
impl WithDeserializer for RiseSetElementType {
    type Deserializer = quick_xml_deserialize::RiseSetElementTypeDeserializer;
}
#[derive(Debug)]
pub struct RiseSetType {
    pub disclaimer: Option<::std::string::String>,
    pub date_time: Vec<DateStampType>,
}
impl WithDeserializer for RiseSetType {
    type Deserializer = quick_xml_deserialize::RiseSetTypeDeserializer;
}
#[derive(Debug)]
pub enum SecondType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for SecondType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
pub type SiteData = SiteDataElementType;
#[derive(Debug)]
pub struct SiteDataElementType {
    pub license: ::std::string::String,
    pub date_time: [DateStampType; 2usize],
    pub location: LocationType,
    pub warnings: WarningsType,
    pub current_conditions: CurrentConditionsType,
    pub forecast_group: ForecastGroupTypeFullType,
    pub hourly_forecast_group: HourlyForecastGroupTypeFullType,
    pub rise_set: RiseSetType,
}
impl WithDeserializer for SiteDataElementType {
    type Deserializer = quick_xml_deserialize::SiteDataElementTypeDeserializer;
}
pub type SiteList = SiteListElementType;
#[derive(Debug)]
pub struct SiteListElementType {
    pub site: Vec<SiteType>,
}
impl WithDeserializer for SiteListElementType {
    type Deserializer = quick_xml_deserialize::SiteListElementTypeDeserializer;
}
#[derive(Debug)]
pub struct SiteType {
    pub code: Option<::std::string::String>,
    pub name_en: ::std::string::String,
    pub name_fr: ::std::string::String,
    pub province_code: ::std::string::String,
}
impl WithDeserializer for SiteType {
    type Deserializer = quick_xml_deserialize::SiteTypeDeserializer;
}
#[derive(Debug)]
pub struct SnowLevelType {
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for SnowLevelType {
    type Deserializer = quick_xml_deserialize::SnowLevelTypeDeserializer;
}
#[derive(Debug)]
pub struct StationType {
    pub code: Option<::std::string::String>,
    pub lat: Option<ValidLatLonType>,
    pub lon: Option<ValidLatLonType>,
    pub country: Option<ValidCountryCodeType>,
    pub content: ::std::string::String,
}
impl WithDeserializer for StationType {
    type Deserializer = quick_xml_deserialize::StationTypeDeserializer;
}
#[derive(Debug)]
pub struct TemperatureHourlyType {
    pub units: Option<ValidTemperatureUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub content: ValidTemperaturesType,
}
impl WithDeserializer for TemperatureHourlyType {
    type Deserializer = quick_xml_deserialize::TemperatureHourlyTypeDeserializer;
}
#[derive(Debug)]
pub struct TemperatureType {
    pub units: Option<ValidTemperatureUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub class: Option<ValidTemperatureClassesType>,
    pub year: Option<::std::string::String>,
    pub period: Option<PeriodRangeType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidTemperaturesType,
}
impl WithDeserializer for TemperatureType {
    type Deserializer = quick_xml_deserialize::TemperatureTypeDeserializer;
}
#[derive(Debug)]
pub struct TemperaturesType {
    pub text_summary: ::std::string::String,
    pub temperature: Vec<TemperatureType>,
}
impl WithDeserializer for TemperaturesType {
    type Deserializer = quick_xml_deserialize::TemperaturesTypeDeserializer;
}
pub type TextSummaryType = ::std::string::String;
#[derive(Debug)]
pub struct TimeType {
    pub name: Option<::std::string::String>,
    pub zone: Option<ValidTimeZonesType>,
    pub hour: HourType,
    pub minute: MinuteType,
    pub second: Option<SecondType>,
}
impl WithDeserializer for TimeType {
    type Deserializer = quick_xml_deserialize::TimeTypeDeserializer;
}
#[derive(Debug)]
pub enum UnitsHourlyType {
    Cardinal,
}
impl DeserializeBytes for UnitsHourlyType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"cardinal" => Ok(Self::Cardinal),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum UvCategoryType {
    Low,
    Moderate,
    High,
    VeryHigh,
    Extreme,
    Bas,
    Modéré,
    élevé,
    Trèsélevé,
    Extrême,
    None,
}
impl DeserializeBytes for UvCategoryType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"low" => Ok(Self::Low),
            b"moderate" => Ok(Self::Moderate),
            b"high" => Ok(Self::High),
            b"very high" => Ok(Self::VeryHigh),
            b"extreme" => Ok(Self::Extreme),
            b"bas" => Ok(Self::Bas),
            b"mod\xC3\xA9r\xC3\xA9" => Ok(Self::Modéré),
            b"\xC3\xA9lev\xC3\xA9" => Ok(Self::élevé),
            b"tr\xC3\xA8s \xC3\xA9lev\xC3\xA9" => Ok(Self::Trèsélevé),
            b"extr\xC3\xAAme" => Ok(Self::Extrême),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct UvHourlyType {
    pub index: UvIndexType,
}
impl WithDeserializer for UvHourlyType {
    type Deserializer = quick_xml_deserialize::UvHourlyTypeDeserializer;
}
#[derive(Debug)]
pub enum UvIndexType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for UvIndexType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub struct UvType {
    pub category: Option<UvCategoryType>,
    pub index: UvIndexType,
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for UvType {
    type Deserializer = quick_xml_deserialize::UvTypeDeserializer;
}
#[derive(Debug)]
pub enum ValidAccumulationNamesType {
    Snow,
    Rain,
    Freezing,
    IcePellets,
    FreezingRain,
    Neige,
    Pluie,
    Verglaçante,
    Grésil,
    PluieVerglasçante,
}
impl DeserializeBytes for ValidAccumulationNamesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"snow" => Ok(Self::Snow),
            b"rain" => Ok(Self::Rain),
            b"freezing" => Ok(Self::Freezing),
            b"ice pellets" => Ok(Self::IcePellets),
            b"freezing rain" => Ok(Self::FreezingRain),
            b"neige" => Ok(Self::Neige),
            b"pluie" => Ok(Self::Pluie),
            b"vergla\xC3\xA7ante" => Ok(Self::Verglaçante),
            b"gr\xC3\xA9sil" => Ok(Self::Grésil),
            b"pluie verglas\xC3\xA7ante" => Ok(Self::PluieVerglasçante),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidCountryCodeType {
    Ca,
    Na,
}
impl DeserializeBytes for ValidCountryCodeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"CA" => Ok(Self::Ca),
            b"NA" => Ok(Self::Na),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidDayNamesType {
    Today,
    Tonight,
    Monday,
    MondayNight,
    Tuesday,
    TuesdayNight,
    Wednesday,
    WednesdayNight,
    Thursday,
    ThursdayNight,
    Friday,
    FridayNight,
    Saturday,
    SaturdayNight,
    Sunday,
    SundayNight,
    AujourdHui,
    CeSoirEtCetteNuit,
    ceSoirEtCetteNuit,
    Lundi,
    LundiSoirEtNuit,
    lundiSoirEtNuit,
    Mardi,
    MardiSoirEtNuit,
    mardiSoirEtNuit,
    Mercredi,
    MercrediSoirEtNuit,
    mercrediSoirEtNuit,
    Jeudi,
    JeudiSoirEtNuit,
    jeudiSoirEtNuit,
    Vendredi,
    VendrediSoirEtNuit,
    vendrediSoirEtNuit,
    Samedi,
    SamediSoirEtNuit,
    samediSoirEtNuit,
    Dimanche,
    DimancheSoirEtNuit,
    dimancheSoirEtNuit,
    lundi,
    mardi,
    mercredi,
    jeudi,
    vendredi,
    samedi,
    dima,
}
impl DeserializeBytes for ValidDayNamesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"Today" => Ok(Self::Today),
            b"Tonight" => Ok(Self::Tonight),
            b"Monday" => Ok(Self::Monday),
            b"Monday night" => Ok(Self::MondayNight),
            b"Tuesday" => Ok(Self::Tuesday),
            b"Tuesday night" => Ok(Self::TuesdayNight),
            b"Wednesday" => Ok(Self::Wednesday),
            b"Wednesday night" => Ok(Self::WednesdayNight),
            b"Thursday" => Ok(Self::Thursday),
            b"Thursday night" => Ok(Self::ThursdayNight),
            b"Friday" => Ok(Self::Friday),
            b"Friday night" => Ok(Self::FridayNight),
            b"Saturday" => Ok(Self::Saturday),
            b"Saturday night" => Ok(Self::SaturdayNight),
            b"Sunday" => Ok(Self::Sunday),
            b"Sunday night" => Ok(Self::SundayNight),
            b"Aujourd'hui" => Ok(Self::AujourdHui),
            b"Ce soir et cette nuit" => Ok(Self::CeSoirEtCetteNuit),
            b"ce soir et cette nuit" => Ok(Self::ceSoirEtCetteNuit),
            b"Lundi" => Ok(Self::Lundi),
            b"Lundi soir et nuit" => Ok(Self::LundiSoirEtNuit),
            b"lundi soir et nuit" => Ok(Self::lundiSoirEtNuit),
            b"Mardi" => Ok(Self::Mardi),
            b"Mardi soir et nuit" => Ok(Self::MardiSoirEtNuit),
            b"mardi soir et nuit" => Ok(Self::mardiSoirEtNuit),
            b"Mercredi" => Ok(Self::Mercredi),
            b"Mercredi soir et nuit" => Ok(Self::MercrediSoirEtNuit),
            b"mercredi soir et nuit" => Ok(Self::mercrediSoirEtNuit),
            b"Jeudi" => Ok(Self::Jeudi),
            b"Jeudi soir et nuit" => Ok(Self::JeudiSoirEtNuit),
            b"jeudi soir et nuit" => Ok(Self::jeudiSoirEtNuit),
            b"Vendredi" => Ok(Self::Vendredi),
            b"Vendredi soir et nuit" => Ok(Self::VendrediSoirEtNuit),
            b"vendredi soir et nuit" => Ok(Self::vendrediSoirEtNuit),
            b"Samedi" => Ok(Self::Samedi),
            b"Samedi soir et nuit" => Ok(Self::SamediSoirEtNuit),
            b"samedi soir et nuit" => Ok(Self::samediSoirEtNuit),
            b"Dimanche" => Ok(Self::Dimanche),
            b"Dimanche soir et nuit" => Ok(Self::DimancheSoirEtNuit),
            b"dimanche soir et nuit" => Ok(Self::dimancheSoirEtNuit),
            b"lundi" => Ok(Self::lundi),
            b"mardi" => Ok(Self::mardi),
            b"mercredi" => Ok(Self::mercredi),
            b"jeudi" => Ok(Self::jeudi),
            b"vendredi" => Ok(Self::vendredi),
            b"samedi" => Ok(Self::samedi),
            b"dimanche" => Ok(Self::dima),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub type ValidDayNumbersType = ::core::primitive::usize;
#[derive(Debug)]
pub enum ValidFormatType {
    Png,
}
impl DeserializeBytes for ValidFormatType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"png" => Ok(Self::Png),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidHoursType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for ValidHoursType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidHumidexType {
    None,
    F32(::core::primitive::f32),
    I32(::core::primitive::i32),
}
impl DeserializeBytes for ValidHumidexType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidIconCodesType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for ValidIconCodesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidIconCodesUnitsType {
    Gif,
}
impl DeserializeBytes for ValidIconCodesUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"gif" => Ok(Self::Gif),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub type ValidIndexTypesType = ::core::primitive::i32;
#[derive(Debug)]
pub enum ValidLatLonType {
    String(::std::string::String),
    None,
}
impl DeserializeBytes for ValidLatLonType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::String(::std::string::String::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
pub type ValidLocationCodeType = ::std::string::String;
pub type ValidLocationLatLonType = ::std::string::String;
#[derive(Debug)]
pub enum ValidMonthNamesType {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    Janvier,
    Février,
    Mars,
    Avril,
    Mai,
    Juin,
    Juillet,
    Août,
    Septembre,
    Octobre,
    Novembre,
    Décembre,
}
impl DeserializeBytes for ValidMonthNamesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"January" => Ok(Self::January),
            b"February" => Ok(Self::February),
            b"March" => Ok(Self::March),
            b"April" => Ok(Self::April),
            b"May" => Ok(Self::May),
            b"June" => Ok(Self::June),
            b"July" => Ok(Self::July),
            b"August" => Ok(Self::August),
            b"September" => Ok(Self::September),
            b"October" => Ok(Self::October),
            b"November" => Ok(Self::November),
            b"December" => Ok(Self::December),
            b"janvier" => Ok(Self::Janvier),
            b"f\xC3\xA9vrier" => Ok(Self::Février),
            b"mars" => Ok(Self::Mars),
            b"avril" => Ok(Self::Avril),
            b"mai" => Ok(Self::Mai),
            b"juin" => Ok(Self::Juin),
            b"juillet" => Ok(Self::Juillet),
            b"ao\xC3\xBBt" => Ok(Self::Août),
            b"septembre" => Ok(Self::Septembre),
            b"octobre" => Ok(Self::Octobre),
            b"novembre" => Ok(Self::Novembre),
            b"d\xC3\xA9cembre" => Ok(Self::Décembre),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub type ValidMonthNumbersType = ::core::primitive::i32;
#[derive(Debug)]
pub enum ValidPopUnitsType {
    Percent,
    None,
}
impl DeserializeBytes for ValidPopUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"%" => Ok(Self::Percent),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidPopsType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for ValidPopsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidPrecipAbbreviatedCodesType {
    Rain,
    Snow,
    FreezingRain,
    Drizzle,
    RainAndSnow,
    RainAndFreezingRain,
    RainAndDrizzle,
    SnowAndFreezingRain,
    SnowAndDrizzle,
    FreezingRainAndDrizzle,
    Pluie,
    Neige,
    PluieVerglaçante,
    Bruine,
    PluieEtNeige,
    PluieEtPluieVerglaçante,
    PluieEtBruine,
    NeigeEtPluieVerglaçante,
    NeigeEtBruine,
    PluieVerglaçanteEtBruine,
    None,
}
impl DeserializeBytes for ValidPrecipAbbreviatedCodesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"rain" => Ok(Self::Rain),
            b"snow" => Ok(Self::Snow),
            b"freezing rain" => Ok(Self::FreezingRain),
            b"drizzle" => Ok(Self::Drizzle),
            b"rain and snow" => Ok(Self::RainAndSnow),
            b"rain and freezing rain" => Ok(Self::RainAndFreezingRain),
            b"rain and drizzle" => Ok(Self::RainAndDrizzle),
            b"snow and freezing rain" => Ok(Self::SnowAndFreezingRain),
            b"snow and drizzle" => Ok(Self::SnowAndDrizzle),
            b"freezing rain and drizzle" => Ok(Self::FreezingRainAndDrizzle),
            b"pluie" => Ok(Self::Pluie),
            b"neige" => Ok(Self::Neige),
            b"pluie vergla\xC3\xA7ante" => Ok(Self::PluieVerglaçante),
            b"bruine" => Ok(Self::Bruine),
            b"pluie et neige" => Ok(Self::PluieEtNeige),
            b"pluie et pluie vergla\xC3\xA7ante" => Ok(Self::PluieEtPluieVerglaçante),
            b"pluie et bruine" => Ok(Self::PluieEtBruine),
            b"neige et pluie vergla\xC3\xA7ante" => Ok(Self::NeigeEtPluieVerglaçante),
            b"neige et bruine" => Ok(Self::NeigeEtBruine),
            b"pluie vergla\xC3\xA7ante et bruine" => Ok(Self::PluieVerglaçanteEtBruine),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidPrecipClassesType {
    ExtremeRainfall,
    ExtremeSnowfall,
    ExtremePrecipitation,
    ExtremeSnowOnGround,
}
impl DeserializeBytes for ValidPrecipClassesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"extremeRainfall" => Ok(Self::ExtremeRainfall),
            b"extremeSnowfall" => Ok(Self::ExtremeSnowfall),
            b"extremePrecipitation" => Ok(Self::ExtremePrecipitation),
            b"extremeSnowOnGround" => Ok(Self::ExtremeSnowOnGround),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidPrecipSubTypeForecastHoursType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for ValidPrecipSubTypeForecastHoursType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidPrecipUnitsType {
    Mm,
    Cm,
    None,
}
impl DeserializeBytes for ValidPrecipUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"mm" => Ok(Self::Mm),
            b"cm" => Ok(Self::Cm),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidPrecipsType {
    None,
    Trace,
    F64(::core::primitive::f64),
}
impl DeserializeBytes for ValidPrecipsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            b"Trace" => Ok(Self::Trace),
            x => Ok(Self::F64(::core::primitive::f64::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidPressureTendenciesType {
    Rising,
    Falling,
    Steady,
    àLaHausse,
    àLaBaisse,
    Stationnaire,
    None,
}
impl DeserializeBytes for ValidPressureTendenciesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"rising" => Ok(Self::Rising),
            b"falling" => Ok(Self::Falling),
            b"steady" => Ok(Self::Steady),
            b"\xC3\xA0 la hausse" => Ok(Self::àLaHausse),
            b"\xC3\xA0 la baisse" => Ok(Self::àLaBaisse),
            b"stationnaire" => Ok(Self::Stationnaire),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidPressureUnitsType {
    KPa,
    None,
}
impl DeserializeBytes for ValidPressureUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"kPa" => Ok(Self::KPa),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidPressuresType {
    None,
    F64(::core::primitive::f64),
}
impl DeserializeBytes for ValidPressuresType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::F64(::core::primitive::f64::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidRelativeHumiditiesType {
    None,
    I32(::core::primitive::i32),
    F32(::core::primitive::f32),
}
impl DeserializeBytes for ValidRelativeHumiditiesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::F32(::core::primitive::f32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidRelativeHumidityUnitsType {
    Percent,
    None,
}
impl DeserializeBytes for ValidRelativeHumidityUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"%" => Ok(Self::Percent),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub type ValidStationCodeType = ::std::string::String;
#[derive(Debug)]
pub enum ValidTemperatureClassesType {
    High,
    Low,
    Max,
    Min,
    ExtremeMax,
    ExtremeMin,
    NormalMax,
    NormalMin,
    NormalMean,
    Mean,
    Current,
    Dewpoint,
}
impl DeserializeBytes for ValidTemperatureClassesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"high" => Ok(Self::High),
            b"low" => Ok(Self::Low),
            b"max" => Ok(Self::Max),
            b"min" => Ok(Self::Min),
            b"extremeMax" => Ok(Self::ExtremeMax),
            b"extremeMin" => Ok(Self::ExtremeMin),
            b"normalMax" => Ok(Self::NormalMax),
            b"normalMin" => Ok(Self::NormalMin),
            b"normalMean" => Ok(Self::NormalMean),
            b"mean" => Ok(Self::Mean),
            b"current" => Ok(Self::Current),
            b"dewpoint" => Ok(Self::Dewpoint),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidTemperatureUnitsType {
    C,
    None,
}
impl DeserializeBytes for ValidTemperatureUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"C" => Ok(Self::C),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidTemperaturesType {
    None,
    F64(::core::primitive::f64),
}
impl DeserializeBytes for ValidTemperaturesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::F64(::core::primitive::f64::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidTimeZonesType {
    Adt,
    Ast,
    Cdt,
    Cst,
    Edt,
    Est,
    Mdt,
    Mst,
    Nst,
    Ndt,
    Pdt,
    Pst,
    Haa,
    Hna,
    Hac,
    Hnc,
    Hae,
    Hne,
    Har,
    Hnr,
    Hat,
    Hnt,
    Hap,
    Hnp,
    Utc,
}
impl DeserializeBytes for ValidTimeZonesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"ADT" => Ok(Self::Adt),
            b"AST" => Ok(Self::Ast),
            b"CDT" => Ok(Self::Cdt),
            b"CST" => Ok(Self::Cst),
            b"EDT" => Ok(Self::Edt),
            b"EST" => Ok(Self::Est),
            b"MDT" => Ok(Self::Mdt),
            b"MST" => Ok(Self::Mst),
            b"NST" => Ok(Self::Nst),
            b"NDT" => Ok(Self::Ndt),
            b"PDT" => Ok(Self::Pdt),
            b"PST" => Ok(Self::Pst),
            b"HAA" => Ok(Self::Haa),
            b"HNA" => Ok(Self::Hna),
            b"HAC" => Ok(Self::Hac),
            b"HNC" => Ok(Self::Hnc),
            b"HAE" => Ok(Self::Hae),
            b"HNE" => Ok(Self::Hne),
            b"HAR" => Ok(Self::Har),
            b"HNR" => Ok(Self::Hnr),
            b"HAT" => Ok(Self::Hat),
            b"HNT" => Ok(Self::Hnt),
            b"HAP" => Ok(Self::Hap),
            b"HNP" => Ok(Self::Hnp),
            b"UTC" => Ok(Self::Utc),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub type ValidUtcOffsetType = ::core::primitive::f64;
#[derive(Debug)]
pub enum ValidUnitTypesType {
    Imperial,
    Metric,
    None,
}
impl DeserializeBytes for ValidUnitTypesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"imperial" => Ok(Self::Imperial),
            b"metric" => Ok(Self::Metric),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidVisibilitiesType {
    None,
    F64(::core::primitive::f64),
}
impl DeserializeBytes for ValidVisibilitiesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::F64(::core::primitive::f64::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidVisibilityUnitsType {
    Km,
    None,
}
impl DeserializeBytes for ValidVisibilityUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"km" => Ok(Self::Km),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub type ValidWarningCodesType = ::core::primitive::i32;
#[derive(Debug)]
pub enum ValidWarningPrioritiesType {
    Urgent,
    High,
    Medium,
    Low,
}
impl DeserializeBytes for ValidWarningPrioritiesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"urgent" => Ok(Self::Urgent),
            b"high" => Ok(Self::High),
            b"medium" => Ok(Self::Medium),
            b"low" => Ok(Self::Low),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidWarningTypesType {
    Advisory,
    Warning,
    Watch,
    Ended,
    Statement,
    None,
}
impl DeserializeBytes for ValidWarningTypesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"advisory" => Ok(Self::Advisory),
            b"warning" => Ok(Self::Warning),
            b"watch" => Ok(Self::Watch),
            b"ended" => Ok(Self::Ended),
            b"statement" => Ok(Self::Statement),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindBearingUnitsType {
    Degrees,
}
impl DeserializeBytes for ValidWindBearingUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"degrees" => Ok(Self::Degrees),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindBearingsType {
    None,
    F32(::core::primitive::f32),
}
impl DeserializeBytes for ValidWindBearingsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::F32(::core::primitive::f32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindChillClassType {
    Morning,
    Afternoon,
    Evening,
    Overnight,
    Near,
    Matin,
    AprèsMidi,
    Soir,
    Nuit,
    PrèsDe,
    None,
}
impl DeserializeBytes for ValidWindChillClassType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"morning" => Ok(Self::Morning),
            b"afternoon" => Ok(Self::Afternoon),
            b"evening" => Ok(Self::Evening),
            b"overnight" => Ok(Self::Overnight),
            b"near" => Ok(Self::Near),
            b"matin" => Ok(Self::Matin),
            b"apr\xC3\xA8s-midi" => Ok(Self::AprèsMidi),
            b"soir" => Ok(Self::Soir),
            b"nuit" => Ok(Self::Nuit),
            b"pr\xC3\xA8s de" => Ok(Self::PrèsDe),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindChillsType {
    None,
    I32(::core::primitive::i32),
}
impl DeserializeBytes for ValidWindChillsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::I32(::core::primitive::i32::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindDirectionsType {
    N,
    Nne,
    Ne,
    Ene,
    E,
    Ese,
    Se,
    Sse,
    S,
    Ssw,
    Sso,
    Sw,
    So,
    Wsw,
    Oso,
    W,
    O,
    Wnw,
    Ono,
    Nw,
    No,
    Nnw,
    Nno,
    Vr,
    None,
}
impl DeserializeBytes for ValidWindDirectionsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"N" => Ok(Self::N),
            b"NNE" => Ok(Self::Nne),
            b"NE" => Ok(Self::Ne),
            b"ENE" => Ok(Self::Ene),
            b"E" => Ok(Self::E),
            b"ESE" => Ok(Self::Ese),
            b"SE" => Ok(Self::Se),
            b"SSE" => Ok(Self::Sse),
            b"S" => Ok(Self::S),
            b"SSW" => Ok(Self::Ssw),
            b"SSO" => Ok(Self::Sso),
            b"SW" => Ok(Self::Sw),
            b"SO" => Ok(Self::So),
            b"WSW" => Ok(Self::Wsw),
            b"OSO" => Ok(Self::Oso),
            b"W" => Ok(Self::W),
            b"O" => Ok(Self::O),
            b"WNW" => Ok(Self::Wnw),
            b"ONO" => Ok(Self::Ono),
            b"NW" => Ok(Self::Nw),
            b"NO" => Ok(Self::No),
            b"NNW" => Ok(Self::Nnw),
            b"NNO" => Ok(Self::Nno),
            b"VR" => Ok(Self::Vr),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct ValidWindHourlyDirectionsType {
    pub units: Option<UnitsHourlyType>,
    pub wind_dir_full: Option<WindDirFullType>,
    pub content: ValidWindDirectionsType,
}
impl WithDeserializer for ValidWindHourlyDirectionsType {
    type Deserializer = quick_xml_deserialize::ValidWindHourlyDirectionsTypeDeserializer;
}
#[derive(Debug)]
pub enum ValidWindIndicesType {
    None,
    Usize(::core::primitive::usize),
}
impl DeserializeBytes for ValidWindIndicesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            x => Ok(Self::Usize(::core::primitive::usize::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindRanksType {
    Major,
    Minor,
    None,
}
impl DeserializeBytes for ValidWindRanksType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"major" => Ok(Self::Major),
            b"minor" => Ok(Self::Minor),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindSpeedsType {
    None,
    Calm,
    F64(::core::primitive::f64),
}
impl DeserializeBytes for ValidWindSpeedsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"" => Ok(Self::None),
            b"calm" => Ok(Self::Calm),
            x => Ok(Self::F64(::core::primitive::f64::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug)]
pub enum ValidWindUnitsType {
    KmH,
    None,
}
impl DeserializeBytes for ValidWindUnitsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"km/h" => Ok(Self::KmH),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct VisibilitySubTypeForecastType {
    pub cause: Option<::std::string::String>,
    pub text_summary: ::std::string::String,
}
impl WithDeserializer for VisibilitySubTypeForecastType {
    type Deserializer = quick_xml_deserialize::VisibilitySubTypeForecastTypeDeserializer;
}
#[derive(Debug)]
pub struct VisibilityTypeCondType {
    pub units: Option<ValidVisibilityUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidVisibilitiesType,
}
impl WithDeserializer for VisibilityTypeCondType {
    type Deserializer = quick_xml_deserialize::VisibilityTypeCondTypeDeserializer;
}
#[derive(Debug)]
pub struct VisibilityTypeForecastType {
    pub wind_visib: Option<VisibilitySubTypeForecastType>,
    pub other_visib: Option<VisibilitySubTypeForecastType>,
}
impl WithDeserializer for VisibilityTypeForecastType {
    type Deserializer = quick_xml_deserialize::VisibilityTypeForecastTypeDeserializer;
}
#[derive(Debug)]
pub struct WarningEventType {
    pub type_: Option<ValidWarningTypesType>,
    pub description: Option<::std::string::String>,
    pub priority: Option<ValidWarningPrioritiesType>,
    pub expiry_time: Option<TimeStampType>,
    pub url: Option<::std::string::String>,
    pub date_time: [DateStampType; 2usize],
}
impl WithDeserializer for WarningEventType {
    type Deserializer = quick_xml_deserialize::WarningEventTypeDeserializer;
}
#[derive(Debug)]
pub struct WarningsType {
    pub url: Option<::std::string::String>,
    pub event: Option<WarningEventType>,
}
impl WithDeserializer for WarningsType {
    type Deserializer = quick_xml_deserialize::WarningsTypeDeserializer;
}
#[derive(Debug)]
pub struct WindBearingType {
    pub units: Option<ValidWindBearingUnitsType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidWindBearingsType,
}
impl WithDeserializer for WindBearingType {
    type Deserializer = quick_xml_deserialize::WindBearingTypeDeserializer;
}
#[derive(Debug)]
pub struct WindChillHourlyType {
    pub unit_type: Option<ValidUnitTypesType>,
    pub content: ValidWindChillsType,
}
impl WithDeserializer for WindChillHourlyType {
    type Deserializer = quick_xml_deserialize::WindChillHourlyTypeDeserializer;
}
#[derive(Debug)]
pub struct WindChillType {
    pub text_summary: Option<::std::string::String>,
    pub calculated: Vec<CalculatedWindChillType>,
    pub frostbite: Option<FrostbiteWindChillType>,
}
impl WithDeserializer for WindChillType {
    type Deserializer = quick_xml_deserialize::WindChillTypeDeserializer;
}
#[derive(Debug)]
pub enum WindDirFullType {
    North,
    Nord,
    NorthNortheast,
    NordNordEst,
    Northeast,
    NordEst,
    EastNortheast,
    EstNordEst,
    East,
    Est,
    EastSoutheast,
    EstSudEst,
    Southeast,
    SudEst,
    SouthSoutheast,
    SudSudEst,
    South,
    Sud,
    SouthSouthwest,
    SudSudOuest,
    Southwest,
    SudOuest,
    WestSouthwest,
    OuestSudOuest,
    West,
    Ouest,
    WestNorthwest,
    OuestNordOuest,
    Northwest,
    NordOuest,
    NorthNorthwest,
    NordNordOuest,
    VariableDirection,
    DirectionVariable,
    None,
}
impl DeserializeBytes for WindDirFullType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"North" => Ok(Self::North),
            b"Nord" => Ok(Self::Nord),
            b"North-northeast" => Ok(Self::NorthNortheast),
            b"Nord-nord-est" => Ok(Self::NordNordEst),
            b"Northeast" => Ok(Self::Northeast),
            b"Nord-est" => Ok(Self::NordEst),
            b"East-northeast" => Ok(Self::EastNortheast),
            b"Est-nord-est" => Ok(Self::EstNordEst),
            b"East" => Ok(Self::East),
            b"Est" => Ok(Self::Est),
            b"East-southeast" => Ok(Self::EastSoutheast),
            b"Est-sud-est" => Ok(Self::EstSudEst),
            b"Southeast" => Ok(Self::Southeast),
            b"Sud-est" => Ok(Self::SudEst),
            b"South-southeast" => Ok(Self::SouthSoutheast),
            b"Sud-sud-est" => Ok(Self::SudSudEst),
            b"South" => Ok(Self::South),
            b"Sud" => Ok(Self::Sud),
            b"South-southwest" => Ok(Self::SouthSouthwest),
            b"Sud-sud-ouest" => Ok(Self::SudSudOuest),
            b"Southwest" => Ok(Self::Southwest),
            b"Sud-ouest" => Ok(Self::SudOuest),
            b"West-southwest" => Ok(Self::WestSouthwest),
            b"Ouest-sud-ouest" => Ok(Self::OuestSudOuest),
            b"West" => Ok(Self::West),
            b"Ouest" => Ok(Self::Ouest),
            b"West-northwest" => Ok(Self::WestNorthwest),
            b"Ouest-nord-ouest" => Ok(Self::OuestNordOuest),
            b"Northwest" => Ok(Self::Northwest),
            b"Nord-ouest" => Ok(Self::NordOuest),
            b"North-northwest" => Ok(Self::NorthNorthwest),
            b"Nord-nord-ouest" => Ok(Self::NordNordOuest),
            b"Variable direction" => Ok(Self::VariableDirection),
            b"Direction variable" => Ok(Self::DirectionVariable),
            b"" => Ok(Self::None),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct WindDirectionType {
    pub qa_value: Option<QaValueType>,
    pub content: ValidWindDirectionsType,
}
impl WithDeserializer for WindDirectionType {
    type Deserializer = quick_xml_deserialize::WindDirectionTypeDeserializer;
}
#[derive(Debug)]
pub struct WindHourlyType {
    pub speed: WindSpeedType,
    pub direction: ValidWindHourlyDirectionsType,
    pub gust: WindSpeedType,
}
impl WithDeserializer for WindHourlyType {
    type Deserializer = quick_xml_deserialize::WindHourlyTypeDeserializer;
}
#[derive(Debug)]
pub struct WindSpeedType {
    pub units: Option<ValidWindUnitsType>,
    pub unit_type: Option<ValidUnitTypesType>,
    pub qa_value: Option<QaValueType>,
    pub content: ValidWindSpeedsType,
}
impl WithDeserializer for WindSpeedType {
    type Deserializer = quick_xml_deserialize::WindSpeedTypeDeserializer;
}
#[derive(Debug)]
pub struct WindType {
    pub index: Option<ValidWindIndicesType>,
    pub rank: Option<ValidWindRanksType>,
    pub speed: WindSpeedType,
    pub gust: WindSpeedType,
    pub direction: WindDirectionType,
    pub bearing: WindBearingType,
}
impl WithDeserializer for WindType {
    type Deserializer = quick_xml_deserialize::WindTypeDeserializer;
}
#[derive(Debug)]
pub struct WindsType {
    pub text_summary: Option<::std::string::String>,
    pub wind: Vec<WindType>,
}
impl WithDeserializer for WindsType {
    type Deserializer = quick_xml_deserialize::WindsTypeDeserializer;
}
pub type YearType = ::std::string::String;
pub mod quick_xml_deserialize {
    use crate::schemas::{DateTimeUtcType, TimeStampType};
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, ContentDeserializer, DeserializeReader, Deserializer,
        DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
        ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct AbbreviatedForecastTypeDeserializer {
        icon_code: Option<super::IconCodeType>,
        pop: Option<super::PopType>,
        text_summary: Option<::std::string::String>,
        state: Box<AbbreviatedForecastTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AbbreviatedForecastTypeDeserializerState {
        Init__,
        IconCode(Option<<super::IconCodeType as WithDeserializer>::Deserializer>),
        Pop(Option<<super::PopType as WithDeserializer>::Deserializer>),
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AbbreviatedForecastTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                icon_code: None,
                pop: None,
                text_summary: None,
                state: Box::new(AbbreviatedForecastTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AbbreviatedForecastTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AbbreviatedForecastTypeDeserializerState as S;
            match state {
                S::IconCode(Some(deserializer)) => {
                    self.store_icon_code(deserializer.finish(reader)?)?
                }
                S::Pop(Some(deserializer)) => self.store_pop(deserializer.finish(reader)?)?,
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_icon_code(&mut self, value: super::IconCodeType) -> Result<(), Error> {
            if self.icon_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"iconCode",
                )))?;
            }
            self.icon_code = Some(value);
            Ok(())
        }
        fn store_pop(&mut self, value: super::PopType) -> Result<(), Error> {
            if self.pop.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pop")))?;
            }
            self.pop = Some(value);
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_icon_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IconCodeType>,
            fallback: &mut Option<AbbreviatedForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.icon_code.is_some() {
                    fallback
                        .get_or_insert(AbbreviatedForecastTypeDeserializerState::IconCode(None));
                    *self.state = AbbreviatedForecastTypeDeserializerState::Pop(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = AbbreviatedForecastTypeDeserializerState::IconCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_icon_code(data)?;
                    *self.state = AbbreviatedForecastTypeDeserializerState::Pop(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                AbbreviatedForecastTypeDeserializerState::IconCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = AbbreviatedForecastTypeDeserializerState::Pop(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = AbbreviatedForecastTypeDeserializerState::IconCode(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_pop<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PopType>,
            fallback: &mut Option<AbbreviatedForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.pop.is_some() {
                    fallback.get_or_insert(AbbreviatedForecastTypeDeserializerState::Pop(None));
                    *self.state = AbbreviatedForecastTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = AbbreviatedForecastTypeDeserializerState::Pop(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pop(data)?;
                    *self.state = AbbreviatedForecastTypeDeserializerState::TextSummary(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AbbreviatedForecastTypeDeserializerState::Pop(
                                Some(deserializer),
                            ));
                            *self.state =
                                AbbreviatedForecastTypeDeserializerState::TextSummary(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AbbreviatedForecastTypeDeserializerState::Pop(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<AbbreviatedForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback
                        .get_or_insert(AbbreviatedForecastTypeDeserializerState::TextSummary(None));
                    *self.state = AbbreviatedForecastTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = AbbreviatedForecastTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = AbbreviatedForecastTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                AbbreviatedForecastTypeDeserializerState::TextSummary(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = AbbreviatedForecastTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = AbbreviatedForecastTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AbbreviatedForecastType>
        for AbbreviatedForecastTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AbbreviatedForecastType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AbbreviatedForecastType>
        where
            R: DeserializeReader,
        {
            use AbbreviatedForecastTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::IconCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_icon_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Pop(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pop(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = AbbreviatedForecastTypeDeserializerState::IconCode(None);
                        event
                    }
                    (S::IconCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"iconCode", false)?;
                        match self.handle_icon_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Pop(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"pop", false)?;
                        match self.handle_pop(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AbbreviatedForecastType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AbbreviatedForecastTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AbbreviatedForecastType {
                icon_code: self
                    .icon_code
                    .ok_or_else(|| ErrorKind::MissingElement("iconCode".into()))?,
                pop: self
                    .pop
                    .ok_or_else(|| ErrorKind::MissingElement("pop".into()))?,
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AccumulationTypeDeserializer {
        name: Option<super::ValidAccumulationNamesType>,
        amount: Option<super::PrecipType>,
        state: Box<AccumulationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AccumulationTypeDeserializerState {
        Init__,
        Name(Option<<super::ValidAccumulationNamesType as WithDeserializer>::Deserializer>),
        Amount(Option<<super::PrecipType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AccumulationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                name: None,
                amount: None,
                state: Box::new(AccumulationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AccumulationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AccumulationTypeDeserializerState as S;
            match state {
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(reader)?)?,
                S::Amount(Some(deserializer)) => self.store_amount(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_name(&mut self, value: super::ValidAccumulationNamesType) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_amount(&mut self, value: super::PrecipType) -> Result<(), Error> {
            if self.amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"amount",
                )))?;
            }
            self.amount = Some(value);
            Ok(())
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidAccumulationNamesType>,
            fallback: &mut Option<AccumulationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.name.is_some() {
                    fallback.get_or_insert(AccumulationTypeDeserializerState::Name(None));
                    *self.state = AccumulationTypeDeserializerState::Amount(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = AccumulationTypeDeserializerState::Name(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state = AccumulationTypeDeserializerState::Amount(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AccumulationTypeDeserializerState::Name(Some(
                                deserializer,
                            )));
                            *self.state = AccumulationTypeDeserializerState::Amount(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AccumulationTypeDeserializerState::Name(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PrecipType>,
            fallback: &mut Option<AccumulationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.amount.is_some() {
                    fallback.get_or_insert(AccumulationTypeDeserializerState::Amount(None));
                    *self.state = AccumulationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = AccumulationTypeDeserializerState::Amount(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_amount(data)?;
                    *self.state = AccumulationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AccumulationTypeDeserializerState::Amount(
                                Some(deserializer),
                            ));
                            *self.state = AccumulationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AccumulationTypeDeserializerState::Amount(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AccumulationType> for AccumulationTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AccumulationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AccumulationType>
        where
            R: DeserializeReader,
        {
            use AccumulationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Amount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = AccumulationTypeDeserializerState::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"name", false)?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Amount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"amount", false)?;
                        match self.handle_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AccumulationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AccumulationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AccumulationType {
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                amount: self
                    .amount
                    .ok_or_else(|| ErrorKind::MissingElement("amount".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculatedHumidexTypeDeserializer {
        unit_type: Option<super::ValidUnitTypesType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidHumidexType>,
        state: Box<CalculatedHumidexTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CalculatedHumidexTypeDeserializerState {
        Init__,
        Content__(<super::ValidHumidexType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CalculatedHumidexTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                unit_type: unit_type,
                qa_value: qa_value,
                content: None,
                state: Box::new(CalculatedHumidexTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CalculatedHumidexTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CalculatedHumidexTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidHumidexType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidHumidexType>,
        ) -> DeserializerResult<'de, super::CalculatedHumidexType>
        where
            R: DeserializeReader,
        {
            use CalculatedHumidexTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CalculatedHumidexType> for CalculatedHumidexTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CalculatedHumidexType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CalculatedHumidexType>
        where
            R: DeserializeReader,
        {
            use CalculatedHumidexTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CalculatedHumidexType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CalculatedHumidexTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CalculatedHumidexType {
                unit_type: self.unit_type,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculatedWindChillTypeDeserializer {
        index: Option<::core::primitive::i32>,
        unit_type: Option<super::ValidUnitTypesType>,
        class: Option<super::ValidWindChillClassType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidWindChillsType>,
        state: Box<CalculatedWindChillTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CalculatedWindChillTypeDeserializerState {
        Init__,
        Content__(<super::ValidWindChillsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CalculatedWindChillTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut index: Option<::core::primitive::i32> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut class: Option<super::ValidWindChillClassType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"index" {
                    reader.read_attrib(&mut index, b"index", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"class" {
                    reader.read_attrib(&mut class, b"class", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                index: index,
                unit_type: unit_type,
                class: class,
                qa_value: qa_value,
                content: None,
                state: Box::new(CalculatedWindChillTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CalculatedWindChillTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CalculatedWindChillTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidWindChillsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindChillsType>,
        ) -> DeserializerResult<'de, super::CalculatedWindChillType>
        where
            R: DeserializeReader,
        {
            use CalculatedWindChillTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CalculatedWindChillType>
        for CalculatedWindChillTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CalculatedWindChillType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CalculatedWindChillType>
        where
            R: DeserializeReader,
        {
            use CalculatedWindChillTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CalculatedWindChillType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CalculatedWindChillTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CalculatedWindChillType {
                index: self.index,
                unit_type: self.unit_type,
                class: self.class,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CloudPrecipTypeDeserializer {
        text_summary: Option<::std::string::String>,
        state: Box<CloudPrecipTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CloudPrecipTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CloudPrecipTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                state: Box::new(CloudPrecipTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CloudPrecipTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use CloudPrecipTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<CloudPrecipTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(CloudPrecipTypeDeserializerState::TextSummary(None));
                    *self.state = CloudPrecipTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = CloudPrecipTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = CloudPrecipTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(CloudPrecipTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = CloudPrecipTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CloudPrecipTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CloudPrecipType> for CloudPrecipTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::CloudPrecipType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CloudPrecipType>
        where
            R: DeserializeReader,
        {
            use CloudPrecipTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = CloudPrecipTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CloudPrecipType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CloudPrecipTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CloudPrecipType {
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CountryTypeDeserializer {
        code: Option<::std::string::String>,
        content: Option<::std::string::String>,
        state: Box<CountryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CountryTypeDeserializerState {
        Init__,
        Content__(<::std::string::String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CountryTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut code: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"code" {
                    reader.read_attrib(&mut code, b"code", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                code: code,
                content: None,
                state: Box::new(CountryTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CountryTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CountryTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
        ) -> DeserializerResult<'de, super::CountryType>
        where
            R: DeserializeReader,
        {
            use CountryTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CountryType> for CountryTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::CountryType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CountryType>
        where
            R: DeserializeReader,
        {
            use CountryTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CountryType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, CountryTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::CountryType {
                code: self.code,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CurrentConditionsTypeDeserializer {
        station: Option<super::StationType>,
        date_time: Vec<super::DateStampType>,
        condition: Option<::std::string::String>,
        icon_code: Option<super::IconCodeType>,
        temperature: Option<super::TemperatureType>,
        dewpoint: Option<super::TemperatureType>,
        wind_chill: Option<super::CalculatedWindChillType>,
        humidex: Option<super::CalculatedHumidexType>,
        pressure: Option<super::PressureTypeCondType>,
        visibility: Option<super::VisibilityTypeCondType>,
        relative_humidity: Option<super::RelativeHumidityType>,
        wind: Option<super::WindType>,
        state: Box<CurrentConditionsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CurrentConditionsTypeDeserializerState {
        Init__,
        Station(Option<<super::StationType as WithDeserializer>::Deserializer>),
        DateTime(Option<<super::DateStampType as WithDeserializer>::Deserializer>),
        Condition(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        IconCode(Option<<super::IconCodeType as WithDeserializer>::Deserializer>),
        Temperature(Option<<super::TemperatureType as WithDeserializer>::Deserializer>),
        Dewpoint(Option<<super::TemperatureType as WithDeserializer>::Deserializer>),
        WindChill(Option<<super::CalculatedWindChillType as WithDeserializer>::Deserializer>),
        Humidex(Option<<super::CalculatedHumidexType as WithDeserializer>::Deserializer>),
        Pressure(Option<<super::PressureTypeCondType as WithDeserializer>::Deserializer>),
        Visibility(Option<<super::VisibilityTypeCondType as WithDeserializer>::Deserializer>),
        RelativeHumidity(Option<<super::RelativeHumidityType as WithDeserializer>::Deserializer>),
        Wind(Option<<super::WindType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CurrentConditionsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                station: None,
                date_time: Vec::new(),
                condition: None,
                icon_code: None,
                temperature: None,
                dewpoint: None,
                wind_chill: None,
                humidex: None,
                pressure: None,
                visibility: None,
                relative_humidity: None,
                wind: None,
                state: Box::new(CurrentConditionsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CurrentConditionsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use CurrentConditionsTypeDeserializerState as S;
            match state {
                S::Station(Some(deserializer)) => {
                    self.store_station(deserializer.finish(reader)?)?
                }
                S::DateTime(Some(deserializer)) => {
                    self.store_date_time(deserializer.finish(reader)?)?
                }
                S::Condition(Some(deserializer)) => {
                    self.store_condition(deserializer.finish(reader)?)?
                }
                S::IconCode(Some(deserializer)) => {
                    self.store_icon_code(deserializer.finish(reader)?)?
                }
                S::Temperature(Some(deserializer)) => {
                    self.store_temperature(deserializer.finish(reader)?)?
                }
                S::Dewpoint(Some(deserializer)) => {
                    self.store_dewpoint(deserializer.finish(reader)?)?
                }
                S::WindChill(Some(deserializer)) => {
                    self.store_wind_chill(deserializer.finish(reader)?)?
                }
                S::Humidex(Some(deserializer)) => {
                    self.store_humidex(deserializer.finish(reader)?)?
                }
                S::Pressure(Some(deserializer)) => {
                    self.store_pressure(deserializer.finish(reader)?)?
                }
                S::Visibility(Some(deserializer)) => {
                    self.store_visibility(deserializer.finish(reader)?)?
                }
                S::RelativeHumidity(Some(deserializer)) => {
                    self.store_relative_humidity(deserializer.finish(reader)?)?
                }
                S::Wind(Some(deserializer)) => self.store_wind(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_station(&mut self, value: super::StationType) -> Result<(), Error> {
            if self.station.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"station",
                )))?;
            }
            self.station = Some(value);
            Ok(())
        }
        fn store_date_time(&mut self, value: super::DateStampType) -> Result<(), Error> {
            self.date_time.push(value);
            Ok(())
        }
        fn store_condition(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.condition.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"condition",
                )))?;
            }
            self.condition = Some(value);
            Ok(())
        }
        fn store_icon_code(&mut self, value: super::IconCodeType) -> Result<(), Error> {
            if self.icon_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"iconCode",
                )))?;
            }
            self.icon_code = Some(value);
            Ok(())
        }
        fn store_temperature(&mut self, value: super::TemperatureType) -> Result<(), Error> {
            if self.temperature.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"temperature",
                )))?;
            }
            self.temperature = Some(value);
            Ok(())
        }
        fn store_dewpoint(&mut self, value: super::TemperatureType) -> Result<(), Error> {
            if self.dewpoint.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"dewpoint",
                )))?;
            }
            self.dewpoint = Some(value);
            Ok(())
        }
        fn store_wind_chill(&mut self, value: super::CalculatedWindChillType) -> Result<(), Error> {
            if self.wind_chill.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"windChill",
                )))?;
            }
            self.wind_chill = Some(value);
            Ok(())
        }
        fn store_humidex(&mut self, value: super::CalculatedHumidexType) -> Result<(), Error> {
            if self.humidex.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"humidex",
                )))?;
            }
            self.humidex = Some(value);
            Ok(())
        }
        fn store_pressure(&mut self, value: super::PressureTypeCondType) -> Result<(), Error> {
            if self.pressure.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"pressure",
                )))?;
            }
            self.pressure = Some(value);
            Ok(())
        }
        fn store_visibility(&mut self, value: super::VisibilityTypeCondType) -> Result<(), Error> {
            if self.visibility.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"visibility",
                )))?;
            }
            self.visibility = Some(value);
            Ok(())
        }
        fn store_relative_humidity(
            &mut self,
            value: super::RelativeHumidityType,
        ) -> Result<(), Error> {
            if self.relative_humidity.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"relativeHumidity",
                )))?;
            }
            self.relative_humidity = Some(value);
            Ok(())
        }
        fn store_wind(&mut self, value: super::WindType) -> Result<(), Error> {
            if self.wind.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"wind")))?;
            }
            self.wind = Some(value);
            Ok(())
        }
        fn handle_station<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::StationType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Station(None));
                *self.state = CurrentConditionsTypeDeserializerState::DateTime(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_station(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::DateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Station(Some(deserializer)),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::DateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CurrentConditionsTypeDeserializerState::Station(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateStampType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::DateTime(None));
                *self.state = CurrentConditionsTypeDeserializerState::Condition(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time(data)?;
                    if self.date_time.len() < 2usize {
                        *self.state = CurrentConditionsTypeDeserializerState::DateTime(None);
                    } else {
                        *self.state = CurrentConditionsTypeDeserializerState::Condition(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::DateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::DateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::DateTime(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_condition<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Condition(None));
                *self.state = CurrentConditionsTypeDeserializerState::IconCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_condition(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::IconCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Condition(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::IconCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::Condition(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_icon_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IconCodeType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::IconCode(None));
                *self.state = CurrentConditionsTypeDeserializerState::Temperature(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_icon_code(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Temperature(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::IconCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::Temperature(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::IconCode(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_temperature<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TemperatureType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Temperature(None));
                *self.state = CurrentConditionsTypeDeserializerState::Dewpoint(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_temperature(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Dewpoint(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Temperature(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::Dewpoint(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::Temperature(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_dewpoint<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TemperatureType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Dewpoint(None));
                *self.state = CurrentConditionsTypeDeserializerState::WindChill(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_dewpoint(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::WindChill(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Dewpoint(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::WindChill(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::Dewpoint(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_wind_chill<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CalculatedWindChillType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::WindChill(None));
                *self.state = CurrentConditionsTypeDeserializerState::Humidex(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind_chill(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Humidex(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::WindChill(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::Humidex(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::WindChill(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_humidex<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CalculatedHumidexType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Humidex(None));
                *self.state = CurrentConditionsTypeDeserializerState::Pressure(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_humidex(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Pressure(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Humidex(Some(deserializer)),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::Pressure(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CurrentConditionsTypeDeserializerState::Humidex(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_pressure<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PressureTypeCondType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Pressure(None));
                *self.state = CurrentConditionsTypeDeserializerState::Visibility(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pressure(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Visibility(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Pressure(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::Visibility(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::Pressure(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_visibility<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::VisibilityTypeCondType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Visibility(None));
                *self.state = CurrentConditionsTypeDeserializerState::RelativeHumidity(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_visibility(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::RelativeHumidity(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::Visibility(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                CurrentConditionsTypeDeserializerState::RelativeHumidity(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::Visibility(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_relative_humidity<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RelativeHumidityType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::RelativeHumidity(
                    None,
                ));
                *self.state = CurrentConditionsTypeDeserializerState::Wind(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_relative_humidity(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Wind(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CurrentConditionsTypeDeserializerState::RelativeHumidity(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CurrentConditionsTypeDeserializerState::Wind(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CurrentConditionsTypeDeserializerState::RelativeHumidity(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_wind<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindType>,
            fallback: &mut Option<CurrentConditionsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Wind(None));
                *self.state = CurrentConditionsTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind(data)?;
                    *self.state = CurrentConditionsTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(CurrentConditionsTypeDeserializerState::Wind(
                                Some(deserializer),
                            ));
                            *self.state = CurrentConditionsTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CurrentConditionsTypeDeserializerState::Wind(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CurrentConditionsType> for CurrentConditionsTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrentConditionsType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrentConditionsType>
        where
            R: DeserializeReader,
        {
            use CurrentConditionsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Station(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_station(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Condition(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_condition(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IconCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_icon_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Dewpoint(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_dewpoint(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::WindChill(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind_chill(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Humidex(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_humidex(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Pressure(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pressure(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Visibility(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_visibility(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RelativeHumidity(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_relative_humidity(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Wind(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = CurrentConditionsTypeDeserializerState::Station(None);
                        event
                    }
                    (S::Station(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"station", false)?;
                        match self.handle_station(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dateTime", false)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Condition(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"condition", false)?;
                        match self.handle_condition(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IconCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"iconCode", false)?;
                        match self.handle_icon_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"temperature",
                            false,
                        )?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Dewpoint(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dewpoint", false)?;
                        match self.handle_dewpoint(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::WindChill(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"windChill", false)?;
                        match self.handle_wind_chill(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Humidex(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"humidex", false)?;
                        match self.handle_humidex(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Pressure(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"pressure", false)?;
                        match self.handle_pressure(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Visibility(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"visibility",
                            false,
                        )?;
                        match self.handle_visibility(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RelativeHumidity(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"relativeHumidity",
                            false,
                        )?;
                        match self.handle_relative_humidity(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Wind(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"wind", false)?;
                        match self.handle_wind(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CurrentConditionsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CurrentConditionsTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CurrentConditionsType {
                station: self.station,
                date_time: self.date_time,
                condition: self.condition,
                icon_code: self.icon_code,
                temperature: self.temperature,
                dewpoint: self.dewpoint,
                wind_chill: self.wind_chill,
                humidex: self.humidex,
                pressure: self.pressure,
                visibility: self.visibility,
                relative_humidity: self.relative_humidity,
                wind: self.wind,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateStampTypeDeserializer {
        name: Option<super::DateStampNameType>,
        zone: Option<super::ValidTimeZonesType>,
        utc_offset: Option<::core::primitive::f64>,
        year: Option<::std::string::String>,
        month: Option<super::MonthType>,
        day: Option<super::DayType>,
        hour: Option<super::HourType>,
        minute: Option<super::MinuteType>,
        time_stamp: Option<TimeStampType>,
        text_summary: Option<::std::string::String>,
        state: Box<DateStampTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateStampTypeDeserializerState {
        Init__,
        Year(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Month(Option<<super::MonthType as WithDeserializer>::Deserializer>),
        Day(Option<<super::DayType as WithDeserializer>::Deserializer>),
        Hour(Option<<super::HourType as WithDeserializer>::Deserializer>),
        Minute(Option<<super::MinuteType as WithDeserializer>::Deserializer>),
        TimeStamp(Option<<TimeStampType as WithDeserializer>::Deserializer>),
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DateStampTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<super::DateStampNameType> = None;
            let mut zone: Option<super::ValidTimeZonesType> = None;
            let mut utc_offset: Option<::core::primitive::f64> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"zone" {
                    reader.read_attrib(&mut zone, b"zone", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"UTCOffset" {
                    reader.read_attrib(&mut utc_offset, b"UTCOffset", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name,
                zone: zone,
                utc_offset: utc_offset,
                year: None,
                month: None,
                day: None,
                hour: None,
                minute: None,
                time_stamp: None,
                text_summary: None,
                state: Box::new(DateStampTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DateStampTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DateStampTypeDeserializerState as S;
            match state {
                S::Year(Some(deserializer)) => self.store_year(deserializer.finish(reader)?)?,
                S::Month(Some(deserializer)) => self.store_month(deserializer.finish(reader)?)?,
                S::Day(Some(deserializer)) => self.store_day(deserializer.finish(reader)?)?,
                S::Hour(Some(deserializer)) => self.store_hour(deserializer.finish(reader)?)?,
                S::Minute(Some(deserializer)) => self.store_minute(deserializer.finish(reader)?)?,
                S::TimeStamp(Some(deserializer)) => {
                    self.store_time_stamp(deserializer.finish(reader)?)?
                }
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_year(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.year.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"year")))?;
            }
            self.year = Some(value);
            Ok(())
        }
        fn store_month(&mut self, value: super::MonthType) -> Result<(), Error> {
            if self.month.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"month",
                )))?;
            }
            self.month = Some(value);
            Ok(())
        }
        fn store_day(&mut self, value: super::DayType) -> Result<(), Error> {
            if self.day.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"day")))?;
            }
            self.day = Some(value);
            Ok(())
        }
        fn store_hour(&mut self, value: super::HourType) -> Result<(), Error> {
            if self.hour.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"hour")))?;
            }
            self.hour = Some(value);
            Ok(())
        }
        fn store_minute(&mut self, value: super::MinuteType) -> Result<(), Error> {
            if self.minute.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"minute",
                )))?;
            }
            self.minute = Some(value);
            Ok(())
        }
        fn store_time_stamp(&mut self, value: TimeStampType) -> Result<(), Error> {
            if self.time_stamp.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"timeStamp",
                )))?;
            }
            self.time_stamp = Some(value);
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_year<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.year.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::Year(None));
                    *self.state = DateStampTypeDeserializerState::Month(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::Year(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_year(data)?;
                    *self.state = DateStampTypeDeserializerState::Month(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::Year(Some(
                                deserializer,
                            )));
                            *self.state = DateStampTypeDeserializerState::Month(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DateStampTypeDeserializerState::Year(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_month<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MonthType>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.month.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::Month(None));
                    *self.state = DateStampTypeDeserializerState::Day(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::Month(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_month(data)?;
                    *self.state = DateStampTypeDeserializerState::Day(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::Month(Some(
                                deserializer,
                            )));
                            *self.state = DateStampTypeDeserializerState::Day(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DateStampTypeDeserializerState::Month(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_day<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DayType>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.day.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::Day(None));
                    *self.state = DateStampTypeDeserializerState::Hour(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::Day(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_day(data)?;
                    *self.state = DateStampTypeDeserializerState::Hour(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::Day(Some(
                                deserializer,
                            )));
                            *self.state = DateStampTypeDeserializerState::Hour(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DateStampTypeDeserializerState::Day(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_hour<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HourType>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.hour.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::Hour(None));
                    *self.state = DateStampTypeDeserializerState::Minute(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::Hour(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hour(data)?;
                    *self.state = DateStampTypeDeserializerState::Minute(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::Hour(Some(
                                deserializer,
                            )));
                            *self.state = DateStampTypeDeserializerState::Minute(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DateStampTypeDeserializerState::Hour(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_minute<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MinuteType>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.minute.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::Minute(None));
                    *self.state = DateStampTypeDeserializerState::TimeStamp(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::Minute(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_minute(data)?;
                    *self.state = DateStampTypeDeserializerState::TimeStamp(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::Minute(Some(
                                deserializer,
                            )));
                            *self.state = DateStampTypeDeserializerState::TimeStamp(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DateStampTypeDeserializerState::Minute(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_time_stamp<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, TimeStampType>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.time_stamp.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::TimeStamp(None));
                    *self.state = DateStampTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::TimeStamp(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_time_stamp(data)?;
                    *self.state = DateStampTypeDeserializerState::TextSummary(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::TimeStamp(
                                Some(deserializer),
                            ));
                            *self.state = DateStampTypeDeserializerState::TextSummary(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DateStampTypeDeserializerState::TimeStamp(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<DateStampTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(DateStampTypeDeserializerState::TextSummary(None));
                    *self.state = DateStampTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DateStampTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = DateStampTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DateStampTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = DateStampTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DateStampTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DateStampType> for DateStampTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DateStampType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateStampType>
        where
            R: DeserializeReader,
        {
            use DateStampTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Year(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_year(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Month(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_month(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Day(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_day(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Hour(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_hour(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Minute(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_minute(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TimeStamp(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_time_stamp(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = DateStampTypeDeserializerState::Year(None);
                        event
                    }
                    (S::Year(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"year", false)?;
                        match self.handle_year(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Month(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"month", false)?;
                        match self.handle_month(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Day(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"day", false)?;
                        match self.handle_day(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Hour(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"hour", false)?;
                        match self.handle_hour(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Minute(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"minute", false)?;
                        match self.handle_minute(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TimeStamp(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"timeStamp", false)?;
                        match self.handle_time_stamp(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DateStampType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, DateStampTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DateStampType {
                name: self.name,
                zone: self.zone,
                utc_offset: self.utc_offset,
                year: self
                    .year
                    .ok_or_else(|| ErrorKind::MissingElement("year".into()))?,
                month: self
                    .month
                    .ok_or_else(|| ErrorKind::MissingElement("month".into()))?,
                day: self
                    .day
                    .ok_or_else(|| ErrorKind::MissingElement("day".into()))?,
                hour: self
                    .hour
                    .ok_or_else(|| ErrorKind::MissingElement("hour".into()))?,
                minute: self
                    .minute
                    .ok_or_else(|| ErrorKind::MissingElement("minute".into()))?,
                time_stamp: self
                    .time_stamp
                    .ok_or_else(|| ErrorKind::MissingElement("timeStamp".into()))?,
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DayTypeDeserializer {
        name: super::ValidDayNamesType,
        content: Option<::core::primitive::usize>,
        state: Box<DayTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DayTypeDeserializerState {
        Init__,
        Content__(<::core::primitive::usize as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DayTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<super::ValidDayNamesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                content: None,
                state: Box::new(DayTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DayTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DayTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::core::primitive::usize) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::core::primitive::usize>,
        ) -> DeserializerResult<'de, super::DayType>
        where
            R: DeserializeReader,
        {
            use DayTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DayType> for DayTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DayType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DayType>
        where
            R: DeserializeReader,
        {
            use DayTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DayType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, DayTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DayType {
                name: self.name,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ForecastGroupTypeFullTypeDeserializer {
        date_time: Vec<super::DateStampType>,
        regional_normals: Option<super::RegionalNormalsType>,
        forecast: Vec<super::ForecastTypeFullType>,
        state: Box<ForecastGroupTypeFullTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ForecastGroupTypeFullTypeDeserializerState {
        Init__,
        DateTime(Option<<super::DateStampType as WithDeserializer>::Deserializer>),
        RegionalNormals(Option<<super::RegionalNormalsType as WithDeserializer>::Deserializer>),
        Forecast(Option<<super::ForecastTypeFullType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ForecastGroupTypeFullTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                date_time: Vec::new(),
                regional_normals: None,
                forecast: Vec::new(),
                state: Box::new(ForecastGroupTypeFullTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ForecastGroupTypeFullTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ForecastGroupTypeFullTypeDeserializerState as S;
            match state {
                S::DateTime(Some(deserializer)) => {
                    self.store_date_time(deserializer.finish(reader)?)?
                }
                S::RegionalNormals(Some(deserializer)) => {
                    self.store_regional_normals(deserializer.finish(reader)?)?
                }
                S::Forecast(Some(deserializer)) => {
                    self.store_forecast(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_date_time(&mut self, value: super::DateStampType) -> Result<(), Error> {
            self.date_time.push(value);
            Ok(())
        }
        fn store_regional_normals(
            &mut self,
            value: super::RegionalNormalsType,
        ) -> Result<(), Error> {
            if self.regional_normals.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"regionalNormals",
                )))?;
            }
            self.regional_normals = Some(value);
            Ok(())
        }
        fn store_forecast(&mut self, value: super::ForecastTypeFullType) -> Result<(), Error> {
            self.forecast.push(value);
            Ok(())
        }
        fn handle_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateStampType>,
            fallback: &mut Option<ForecastGroupTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.date_time.len() < 2usize {
                    *self.state = ForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback
                        .get_or_insert(ForecastGroupTypeFullTypeDeserializerState::DateTime(None));
                    *self.state = ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time(data)?;
                    if self.date_time.len() < 2usize {
                        *self.state = ForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                    } else {
                        *self.state =
                            ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastGroupTypeFullTypeDeserializerState::DateTime(Some(
                                    deserializer,
                                )),
                            );
                            if self.date_time.len().saturating_add(1) < 2usize {
                                *self.state =
                                    ForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                            } else {
                                *self.state =
                                    ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(
                                        None,
                                    );
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastGroupTypeFullTypeDeserializerState::DateTime(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_regional_normals<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RegionalNormalsType>,
            fallback: &mut Option<ForecastGroupTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.regional_normals.is_some() {
                    fallback.get_or_insert(
                        ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(None),
                    );
                    *self.state = ForecastGroupTypeFullTypeDeserializerState::Forecast(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_regional_normals(data)?;
                    *self.state = ForecastGroupTypeFullTypeDeserializerState::Forecast(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ForecastGroupTypeFullTypeDeserializerState::Forecast(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastGroupTypeFullTypeDeserializerState::RegionalNormals(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_forecast<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ForecastTypeFullType>,
            fallback: &mut Option<ForecastGroupTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastGroupTypeFullTypeDeserializerState::Forecast(None));
                *self.state = ForecastGroupTypeFullTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_forecast(data)?;
                    if self.forecast.len() < 20usize {
                        *self.state = ForecastGroupTypeFullTypeDeserializerState::Forecast(None);
                    } else {
                        *self.state = ForecastGroupTypeFullTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastGroupTypeFullTypeDeserializerState::Forecast(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ForecastGroupTypeFullTypeDeserializerState::Forecast(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastGroupTypeFullTypeDeserializerState::Forecast(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ForecastGroupTypeFullType>
        for ForecastGroupTypeFullTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ForecastGroupTypeFullType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ForecastGroupTypeFullType>
        where
            R: DeserializeReader,
        {
            use ForecastGroupTypeFullTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RegionalNormals(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_regional_normals(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Forecast(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_forecast(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                        event
                    }
                    (S::DateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dateTime", false)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RegionalNormals(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"regionalNormals",
                            false,
                        )?;
                        match self.handle_regional_normals(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Forecast(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"forecast", false)?;
                        match self.handle_forecast(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ForecastGroupTypeFullType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ForecastGroupTypeFullTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ForecastGroupTypeFullType {
                date_time: self.date_time.try_into().map_err(|vec: Vec<_>| {
                    ErrorKind::InsufficientSize {
                        min: 2usize,
                        max: 2usize,
                        actual: vec.len(),
                    }
                })?,
                regional_normals: self
                    .regional_normals
                    .ok_or_else(|| ErrorKind::MissingElement("regionalNormals".into()))?,
                forecast: self.forecast,
            })
        }
    }
    #[derive(Debug)]
    pub struct ForecastTypeFullTypeDeserializer {
        period: Option<super::PeriodType>,
        text_summary: Option<::std::string::String>,
        cloud_precip: Option<super::CloudPrecipType>,
        abbreviated_forecast: Option<super::AbbreviatedForecastType>,
        temperatures: Option<super::TemperaturesType>,
        winds: Option<super::WindsType>,
        precipitation: Option<super::PrecipTypeForecastType>,
        snow_level: Option<super::SnowLevelType>,
        wind_chill: Option<super::WindChillType>,
        visibility: Option<super::VisibilityTypeForecastType>,
        uv: Option<super::UvType>,
        relative_humidity: Option<super::RelativeHumidityType>,
        humidex: Option<super::HumidexType>,
        frost: Option<super::FrostType>,
        state: Box<ForecastTypeFullTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ForecastTypeFullTypeDeserializerState {
        Init__,
        Period(Option<<super::PeriodType as WithDeserializer>::Deserializer>),
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        CloudPrecip(Option<<super::CloudPrecipType as WithDeserializer>::Deserializer>),
        AbbreviatedForecast(
            Option<<super::AbbreviatedForecastType as WithDeserializer>::Deserializer>,
        ),
        Temperatures(Option<<super::TemperaturesType as WithDeserializer>::Deserializer>),
        Winds(Option<<super::WindsType as WithDeserializer>::Deserializer>),
        Precipitation(Option<<super::PrecipTypeForecastType as WithDeserializer>::Deserializer>),
        SnowLevel(Option<<super::SnowLevelType as WithDeserializer>::Deserializer>),
        WindChill(Option<<super::WindChillType as WithDeserializer>::Deserializer>),
        Visibility(Option<<super::VisibilityTypeForecastType as WithDeserializer>::Deserializer>),
        Uv(Option<<super::UvType as WithDeserializer>::Deserializer>),
        RelativeHumidity(Option<<super::RelativeHumidityType as WithDeserializer>::Deserializer>),
        Humidex(Option<<super::HumidexType as WithDeserializer>::Deserializer>),
        Frost(Option<<super::FrostType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ForecastTypeFullTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                period: None,
                text_summary: None,
                cloud_precip: None,
                abbreviated_forecast: None,
                temperatures: None,
                winds: None,
                precipitation: None,
                snow_level: None,
                wind_chill: None,
                visibility: None,
                uv: None,
                relative_humidity: None,
                humidex: None,
                frost: None,
                state: Box::new(ForecastTypeFullTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ForecastTypeFullTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ForecastTypeFullTypeDeserializerState as S;
            match state {
                S::Period(Some(deserializer)) => self.store_period(deserializer.finish(reader)?)?,
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::CloudPrecip(Some(deserializer)) => {
                    self.store_cloud_precip(deserializer.finish(reader)?)?
                }
                S::AbbreviatedForecast(Some(deserializer)) => {
                    self.store_abbreviated_forecast(deserializer.finish(reader)?)?
                }
                S::Temperatures(Some(deserializer)) => {
                    self.store_temperatures(deserializer.finish(reader)?)?
                }
                S::Winds(Some(deserializer)) => self.store_winds(deserializer.finish(reader)?)?,
                S::Precipitation(Some(deserializer)) => {
                    self.store_precipitation(deserializer.finish(reader)?)?
                }
                S::SnowLevel(Some(deserializer)) => {
                    self.store_snow_level(deserializer.finish(reader)?)?
                }
                S::WindChill(Some(deserializer)) => {
                    self.store_wind_chill(deserializer.finish(reader)?)?
                }
                S::Visibility(Some(deserializer)) => {
                    self.store_visibility(deserializer.finish(reader)?)?
                }
                S::Uv(Some(deserializer)) => self.store_uv(deserializer.finish(reader)?)?,
                S::RelativeHumidity(Some(deserializer)) => {
                    self.store_relative_humidity(deserializer.finish(reader)?)?
                }
                S::Humidex(Some(deserializer)) => {
                    self.store_humidex(deserializer.finish(reader)?)?
                }
                S::Frost(Some(deserializer)) => self.store_frost(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_period(&mut self, value: super::PeriodType) -> Result<(), Error> {
            if self.period.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"period",
                )))?;
            }
            self.period = Some(value);
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_cloud_precip(&mut self, value: super::CloudPrecipType) -> Result<(), Error> {
            if self.cloud_precip.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"cloudPrecip",
                )))?;
            }
            self.cloud_precip = Some(value);
            Ok(())
        }
        fn store_abbreviated_forecast(
            &mut self,
            value: super::AbbreviatedForecastType,
        ) -> Result<(), Error> {
            if self.abbreviated_forecast.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"abbreviatedForecast",
                )))?;
            }
            self.abbreviated_forecast = Some(value);
            Ok(())
        }
        fn store_temperatures(&mut self, value: super::TemperaturesType) -> Result<(), Error> {
            if self.temperatures.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"temperatures",
                )))?;
            }
            self.temperatures = Some(value);
            Ok(())
        }
        fn store_winds(&mut self, value: super::WindsType) -> Result<(), Error> {
            if self.winds.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"winds",
                )))?;
            }
            self.winds = Some(value);
            Ok(())
        }
        fn store_precipitation(
            &mut self,
            value: super::PrecipTypeForecastType,
        ) -> Result<(), Error> {
            if self.precipitation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"precipitation",
                )))?;
            }
            self.precipitation = Some(value);
            Ok(())
        }
        fn store_snow_level(&mut self, value: super::SnowLevelType) -> Result<(), Error> {
            if self.snow_level.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"snowLevel",
                )))?;
            }
            self.snow_level = Some(value);
            Ok(())
        }
        fn store_wind_chill(&mut self, value: super::WindChillType) -> Result<(), Error> {
            if self.wind_chill.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"windChill",
                )))?;
            }
            self.wind_chill = Some(value);
            Ok(())
        }
        fn store_visibility(
            &mut self,
            value: super::VisibilityTypeForecastType,
        ) -> Result<(), Error> {
            if self.visibility.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"visibility",
                )))?;
            }
            self.visibility = Some(value);
            Ok(())
        }
        fn store_uv(&mut self, value: super::UvType) -> Result<(), Error> {
            if self.uv.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"uv")))?;
            }
            self.uv = Some(value);
            Ok(())
        }
        fn store_relative_humidity(
            &mut self,
            value: super::RelativeHumidityType,
        ) -> Result<(), Error> {
            if self.relative_humidity.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"relativeHumidity",
                )))?;
            }
            self.relative_humidity = Some(value);
            Ok(())
        }
        fn store_humidex(&mut self, value: super::HumidexType) -> Result<(), Error> {
            if self.humidex.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"humidex",
                )))?;
            }
            self.humidex = Some(value);
            Ok(())
        }
        fn store_frost(&mut self, value: super::FrostType) -> Result<(), Error> {
            if self.frost.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"frost",
                )))?;
            }
            self.frost = Some(value);
            Ok(())
        }
        fn handle_period<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PeriodType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.period.is_some() {
                    fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Period(None));
                    *self.state = ForecastTypeFullTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::Period(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_period(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::TextSummary(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Period(
                                Some(deserializer),
                            ));
                            *self.state = ForecastTypeFullTypeDeserializerState::TextSummary(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::Period(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback
                        .get_or_insert(ForecastTypeFullTypeDeserializerState::TextSummary(None));
                    *self.state = ForecastTypeFullTypeDeserializerState::CloudPrecip(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::CloudPrecip(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::TextSummary(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::CloudPrecip(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::TextSummary(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_cloud_precip<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CloudPrecipType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.cloud_precip.is_some() {
                    fallback
                        .get_or_insert(ForecastTypeFullTypeDeserializerState::CloudPrecip(None));
                    *self.state = ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::CloudPrecip(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_cloud_precip(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::CloudPrecip(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::CloudPrecip(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_abbreviated_forecast<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AbbreviatedForecastType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.abbreviated_forecast.is_some() {
                    fallback.get_or_insert(
                        ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(None),
                    );
                    *self.state = ForecastTypeFullTypeDeserializerState::Temperatures(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_abbreviated_forecast(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Temperatures(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::Temperatures(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::AbbreviatedForecast(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_temperatures<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TemperaturesType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.temperatures.is_some() {
                    fallback
                        .get_or_insert(ForecastTypeFullTypeDeserializerState::Temperatures(None));
                    *self.state = ForecastTypeFullTypeDeserializerState::Winds(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::Temperatures(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_temperatures(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Winds(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::Temperatures(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::Winds(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::Temperatures(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_winds<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindsType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.winds.is_some() {
                    fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Winds(None));
                    *self.state = ForecastTypeFullTypeDeserializerState::Precipitation(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::Winds(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_winds(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Precipitation(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Winds(
                                Some(deserializer),
                            ));
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::Precipitation(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::Winds(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_precipitation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PrecipTypeForecastType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.precipitation.is_some() {
                    fallback
                        .get_or_insert(ForecastTypeFullTypeDeserializerState::Precipitation(None));
                    *self.state = ForecastTypeFullTypeDeserializerState::SnowLevel(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ForecastTypeFullTypeDeserializerState::Precipitation(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_precipitation(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::SnowLevel(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::Precipitation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::SnowLevel(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::Precipitation(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_snow_level<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SnowLevelType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::SnowLevel(None));
                *self.state = ForecastTypeFullTypeDeserializerState::WindChill(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_snow_level(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::WindChill(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::SnowLevel(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::WindChill(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::SnowLevel(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_wind_chill<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindChillType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::WindChill(None));
                *self.state = ForecastTypeFullTypeDeserializerState::Visibility(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind_chill(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Visibility(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::WindChill(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::Visibility(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::WindChill(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_visibility<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::VisibilityTypeForecastType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Visibility(None));
                *self.state = ForecastTypeFullTypeDeserializerState::Uv(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_visibility(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Uv(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::Visibility(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::Uv(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::Visibility(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_uv<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::UvType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Uv(None));
                *self.state = ForecastTypeFullTypeDeserializerState::RelativeHumidity(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_uv(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::RelativeHumidity(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Uv(
                                Some(deserializer),
                            ));
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::RelativeHumidity(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::Uv(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_relative_humidity<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RelativeHumidityType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::RelativeHumidity(
                    None,
                ));
                *self.state = ForecastTypeFullTypeDeserializerState::Humidex(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_relative_humidity(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Humidex(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ForecastTypeFullTypeDeserializerState::RelativeHumidity(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ForecastTypeFullTypeDeserializerState::Humidex(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ForecastTypeFullTypeDeserializerState::RelativeHumidity(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_humidex<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HumidexType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Humidex(None));
                *self.state = ForecastTypeFullTypeDeserializerState::Frost(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_humidex(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Frost(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Humidex(
                                Some(deserializer),
                            ));
                            *self.state = ForecastTypeFullTypeDeserializerState::Frost(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::Humidex(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_frost<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FrostType>,
            fallback: &mut Option<ForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Frost(None));
                *self.state = ForecastTypeFullTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_frost(data)?;
                    *self.state = ForecastTypeFullTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ForecastTypeFullTypeDeserializerState::Frost(
                                Some(deserializer),
                            ));
                            *self.state = ForecastTypeFullTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ForecastTypeFullTypeDeserializerState::Frost(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ForecastTypeFullType> for ForecastTypeFullTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ForecastTypeFullType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ForecastTypeFullType>
        where
            R: DeserializeReader,
        {
            use ForecastTypeFullTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Period(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_period(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CloudPrecip(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_cloud_precip(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::AbbreviatedForecast(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_abbreviated_forecast(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperatures(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_temperatures(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Winds(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_winds(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Precipitation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_precipitation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SnowLevel(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_snow_level(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::WindChill(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind_chill(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Visibility(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_visibility(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Uv(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_uv(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RelativeHumidity(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_relative_humidity(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Humidex(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_humidex(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Frost(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_frost(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ForecastTypeFullTypeDeserializerState::Period(None);
                        event
                    }
                    (S::Period(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"period", false)?;
                        match self.handle_period(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CloudPrecip(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"cloudPrecip",
                            false,
                        )?;
                        match self.handle_cloud_precip(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::AbbreviatedForecast(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"abbreviatedForecast",
                            false,
                        )?;
                        match self.handle_abbreviated_forecast(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperatures(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"temperatures",
                            false,
                        )?;
                        match self.handle_temperatures(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Winds(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"winds", false)?;
                        match self.handle_winds(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Precipitation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"precipitation",
                            false,
                        )?;
                        match self.handle_precipitation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SnowLevel(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"snowLevel", false)?;
                        match self.handle_snow_level(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::WindChill(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"windChill", false)?;
                        match self.handle_wind_chill(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Visibility(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"visibility",
                            false,
                        )?;
                        match self.handle_visibility(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Uv(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"uv", false)?;
                        match self.handle_uv(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RelativeHumidity(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"relativeHumidity",
                            false,
                        )?;
                        match self.handle_relative_humidity(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Humidex(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"humidex", false)?;
                        match self.handle_humidex(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Frost(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"frost", false)?;
                        match self.handle_frost(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ForecastTypeFullType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ForecastTypeFullTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ForecastTypeFullType {
                period: self
                    .period
                    .ok_or_else(|| ErrorKind::MissingElement("period".into()))?,
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
                cloud_precip: self
                    .cloud_precip
                    .ok_or_else(|| ErrorKind::MissingElement("cloudPrecip".into()))?,
                abbreviated_forecast: self
                    .abbreviated_forecast
                    .ok_or_else(|| ErrorKind::MissingElement("abbreviatedForecast".into()))?,
                temperatures: self
                    .temperatures
                    .ok_or_else(|| ErrorKind::MissingElement("temperatures".into()))?,
                winds: self
                    .winds
                    .ok_or_else(|| ErrorKind::MissingElement("winds".into()))?,
                precipitation: self
                    .precipitation
                    .ok_or_else(|| ErrorKind::MissingElement("precipitation".into()))?,
                snow_level: self.snow_level,
                wind_chill: self.wind_chill,
                visibility: self.visibility,
                uv: self.uv,
                relative_humidity: self.relative_humidity,
                humidex: self.humidex,
                frost: self.frost,
            })
        }
    }
    #[derive(Debug)]
    pub struct FrostTypeDeserializer {
        text_summary: Option<::std::string::String>,
        state: Box<FrostTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FrostTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FrostTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                state: Box::new(FrostTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FrostTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FrostTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<FrostTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(FrostTypeDeserializerState::TextSummary(None));
                    *self.state = FrostTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FrostTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = FrostTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FrostTypeDeserializerState::TextSummary(Some(
                                deserializer,
                            )));
                            *self.state = FrostTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FrostTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FrostType> for FrostTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FrostType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FrostType>
        where
            R: DeserializeReader,
        {
            use FrostTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FrostTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FrostType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FrostTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FrostType {
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct HourTypeDeserializer {
        content: Option<super::ValidHoursType>,
        state: Box<HourTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HourTypeDeserializerState {
        Init__,
        Content__(<super::ValidHoursType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl HourTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(HourTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HourTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let HourTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidHoursType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidHoursType>,
        ) -> DeserializerResult<'de, super::HourType>
        where
            R: DeserializeReader,
        {
            use HourTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HourType> for HourTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::HourType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HourType>
        where
            R: DeserializeReader,
        {
            use HourTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HourType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, HourTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::HourType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct HourlyForecastGroupTypeFullTypeDeserializer {
        date_time: Vec<super::DateStampType>,
        hourly_forecast: Vec<super::HourlyForecastTypeFullType>,
        state: Box<HourlyForecastGroupTypeFullTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HourlyForecastGroupTypeFullTypeDeserializerState {
        Init__,
        DateTime(Option<<super::DateStampType as WithDeserializer>::Deserializer>),
        HourlyForecast(
            Option<<super::HourlyForecastTypeFullType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl HourlyForecastGroupTypeFullTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                date_time: Vec::new(),
                hourly_forecast: Vec::new(),
                state: Box::new(HourlyForecastGroupTypeFullTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HourlyForecastGroupTypeFullTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HourlyForecastGroupTypeFullTypeDeserializerState as S;
            match state {
                S::DateTime(Some(deserializer)) => {
                    self.store_date_time(deserializer.finish(reader)?)?
                }
                S::HourlyForecast(Some(deserializer)) => {
                    self.store_hourly_forecast(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_date_time(&mut self, value: super::DateStampType) -> Result<(), Error> {
            self.date_time.push(value);
            Ok(())
        }
        fn store_hourly_forecast(
            &mut self,
            value: super::HourlyForecastTypeFullType,
        ) -> Result<(), Error> {
            self.hourly_forecast.push(value);
            Ok(())
        }
        fn handle_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateStampType>,
            fallback: &mut Option<HourlyForecastGroupTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HourlyForecastGroupTypeFullTypeDeserializerState::DateTime(
                    None,
                ));
                *self.state =
                    HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time(data)?;
                    if self.date_time.len() < 2usize {
                        *self.state =
                            HourlyForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                    } else {
                        *self.state =
                            HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastGroupTypeFullTypeDeserializerState::DateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HourlyForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HourlyForecastGroupTypeFullTypeDeserializerState::DateTime(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_hourly_forecast<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HourlyForecastTypeFullType>,
            fallback: &mut Option<HourlyForecastGroupTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(None),
                );
                *self.state = HourlyForecastGroupTypeFullTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hourly_forecast(data)?;
                    if self.hourly_forecast.len() < 25usize {
                        *self.state =
                            HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(None);
                    } else {
                        *self.state = HourlyForecastGroupTypeFullTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(
                                    None,
                                );
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HourlyForecastGroupTypeFullTypeDeserializerState::HourlyForecast(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HourlyForecastGroupTypeFullType>
        for HourlyForecastGroupTypeFullTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HourlyForecastGroupTypeFullType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HourlyForecastGroupTypeFullType>
        where
            R: DeserializeReader,
        {
            use HourlyForecastGroupTypeFullTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::HourlyForecast(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_hourly_forecast(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            HourlyForecastGroupTypeFullTypeDeserializerState::DateTime(None);
                        event
                    }
                    (S::DateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dateTime", false)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::HourlyForecast(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"hourlyForecast",
                            true,
                        )?;
                        match self.handle_hourly_forecast(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HourlyForecastGroupTypeFullType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HourlyForecastGroupTypeFullTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HourlyForecastGroupTypeFullType {
                date_time: self.date_time,
                hourly_forecast: self.hourly_forecast,
            })
        }
    }
    #[derive(Debug)]
    pub struct HourlyForecastTypeFullTypeDeserializer {
        date_time_utc: Option<DateTimeUtcType>,
        condition: Option<super::xs::AnyType>,
        icon_code: Option<super::IconCodeHourlyType>,
        temperature: Option<super::TemperatureHourlyType>,
        lop: Option<super::LopHourlyType>,
        wind_chill: Option<super::WindChillHourlyType>,
        humidex: Option<super::HumidexHourlyType>,
        wind: Option<super::WindHourlyType>,
        uv: Option<super::UvHourlyType>,
        state: Box<HourlyForecastTypeFullTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HourlyForecastTypeFullTypeDeserializerState {
        Init__,
        Condition(Option<<super::xs::AnyType as WithDeserializer>::Deserializer>),
        IconCode(Option<<super::IconCodeHourlyType as WithDeserializer>::Deserializer>),
        Temperature(Option<<super::TemperatureHourlyType as WithDeserializer>::Deserializer>),
        Lop(Option<<super::LopHourlyType as WithDeserializer>::Deserializer>),
        WindChill(Option<<super::WindChillHourlyType as WithDeserializer>::Deserializer>),
        Humidex(Option<<super::HumidexHourlyType as WithDeserializer>::Deserializer>),
        Wind(Option<<super::WindHourlyType as WithDeserializer>::Deserializer>),
        Uv(Option<<super::UvHourlyType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl HourlyForecastTypeFullTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut date_time_utc: Option<DateTimeUtcType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"dateTimeUTC" {
                    reader.read_attrib(&mut date_time_utc, b"dateTimeUTC", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                date_time_utc: date_time_utc,
                condition: None,
                icon_code: None,
                temperature: None,
                lop: None,
                wind_chill: None,
                humidex: None,
                wind: None,
                uv: None,
                state: Box::new(HourlyForecastTypeFullTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HourlyForecastTypeFullTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HourlyForecastTypeFullTypeDeserializerState as S;
            match state {
                S::Condition(Some(deserializer)) => {
                    self.store_condition(deserializer.finish(reader)?)?
                }
                S::IconCode(Some(deserializer)) => {
                    self.store_icon_code(deserializer.finish(reader)?)?
                }
                S::Temperature(Some(deserializer)) => {
                    self.store_temperature(deserializer.finish(reader)?)?
                }
                S::Lop(Some(deserializer)) => self.store_lop(deserializer.finish(reader)?)?,
                S::WindChill(Some(deserializer)) => {
                    self.store_wind_chill(deserializer.finish(reader)?)?
                }
                S::Humidex(Some(deserializer)) => {
                    self.store_humidex(deserializer.finish(reader)?)?
                }
                S::Wind(Some(deserializer)) => self.store_wind(deserializer.finish(reader)?)?,
                S::Uv(Some(deserializer)) => self.store_uv(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_condition(&mut self, value: super::xs::AnyType) -> Result<(), Error> {
            if self.condition.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"condition",
                )))?;
            }
            self.condition = Some(value);
            Ok(())
        }
        fn store_icon_code(&mut self, value: super::IconCodeHourlyType) -> Result<(), Error> {
            if self.icon_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"iconCode",
                )))?;
            }
            self.icon_code = Some(value);
            Ok(())
        }
        fn store_temperature(&mut self, value: super::TemperatureHourlyType) -> Result<(), Error> {
            if self.temperature.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"temperature",
                )))?;
            }
            self.temperature = Some(value);
            Ok(())
        }
        fn store_lop(&mut self, value: super::LopHourlyType) -> Result<(), Error> {
            if self.lop.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"lop")))?;
            }
            self.lop = Some(value);
            Ok(())
        }
        fn store_wind_chill(&mut self, value: super::WindChillHourlyType) -> Result<(), Error> {
            if self.wind_chill.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"windChill",
                )))?;
            }
            self.wind_chill = Some(value);
            Ok(())
        }
        fn store_humidex(&mut self, value: super::HumidexHourlyType) -> Result<(), Error> {
            if self.humidex.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"humidex",
                )))?;
            }
            self.humidex = Some(value);
            Ok(())
        }
        fn store_wind(&mut self, value: super::WindHourlyType) -> Result<(), Error> {
            if self.wind.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"wind")))?;
            }
            self.wind = Some(value);
            Ok(())
        }
        fn store_uv(&mut self, value: super::UvHourlyType) -> Result<(), Error> {
            if self.uv.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"uv")))?;
            }
            self.uv = Some(value);
            Ok(())
        }
        fn handle_condition<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::xs::AnyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.condition.is_some() {
                    fallback.get_or_insert(HourlyForecastTypeFullTypeDeserializerState::Condition(
                        None,
                    ));
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::IconCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Condition(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_condition(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::IconCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::Condition(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HourlyForecastTypeFullTypeDeserializerState::IconCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Condition(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_icon_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IconCodeHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.icon_code.is_some() {
                    fallback
                        .get_or_insert(HourlyForecastTypeFullTypeDeserializerState::IconCode(None));
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Temperature(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::IconCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_icon_code(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Temperature(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::IconCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HourlyForecastTypeFullTypeDeserializerState::Temperature(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::IconCode(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_temperature<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TemperatureHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.temperature.is_some() {
                    fallback.get_or_insert(
                        HourlyForecastTypeFullTypeDeserializerState::Temperature(None),
                    );
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Lop(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Temperature(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_temperature(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Lop(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::Temperature(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Lop(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Temperature(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_lop<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::LopHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.lop.is_some() {
                    fallback.get_or_insert(HourlyForecastTypeFullTypeDeserializerState::Lop(None));
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::WindChill(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Lop(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_lop(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::WindChill(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::Lop(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HourlyForecastTypeFullTypeDeserializerState::WindChill(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Lop(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_wind_chill<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindChillHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.wind_chill.is_some() {
                    fallback.get_or_insert(HourlyForecastTypeFullTypeDeserializerState::WindChill(
                        None,
                    ));
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Humidex(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::WindChill(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind_chill(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Humidex(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::WindChill(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HourlyForecastTypeFullTypeDeserializerState::Humidex(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::WindChill(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_humidex<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HumidexHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.humidex.is_some() {
                    fallback
                        .get_or_insert(HourlyForecastTypeFullTypeDeserializerState::Humidex(None));
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Wind(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Humidex(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_humidex(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Wind(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::Humidex(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Wind(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Humidex(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_wind<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.wind.is_some() {
                    fallback.get_or_insert(HourlyForecastTypeFullTypeDeserializerState::Wind(None));
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Uv(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Wind(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Uv(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::Wind(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Uv(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Wind(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_uv<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::UvHourlyType>,
            fallback: &mut Option<HourlyForecastTypeFullTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HourlyForecastTypeFullTypeDeserializerState::Uv(None));
                *self.state = HourlyForecastTypeFullTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_uv(data)?;
                    *self.state = HourlyForecastTypeFullTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HourlyForecastTypeFullTypeDeserializerState::Uv(Some(deserializer)),
                            );
                            *self.state = HourlyForecastTypeFullTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HourlyForecastTypeFullTypeDeserializerState::Uv(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HourlyForecastTypeFullType>
        for HourlyForecastTypeFullTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HourlyForecastTypeFullType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HourlyForecastTypeFullType>
        where
            R: DeserializeReader,
        {
            use HourlyForecastTypeFullTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Condition(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_condition(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IconCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_icon_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Lop(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_lop(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::WindChill(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind_chill(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Humidex(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_humidex(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Wind(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Uv(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_uv(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = HourlyForecastTypeFullTypeDeserializerState::Condition(None);
                        event
                    }
                    (S::Condition(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"condition", true)?;
                        match self.handle_condition(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IconCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"iconCode", false)?;
                        match self.handle_icon_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"temperature",
                            false,
                        )?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Lop(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"lop", false)?;
                        match self.handle_lop(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::WindChill(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"windChill", false)?;
                        match self.handle_wind_chill(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Humidex(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"humidex", false)?;
                        match self.handle_humidex(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Wind(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"wind", false)?;
                        match self.handle_wind(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Uv(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"uv", false)?;
                        match self.handle_uv(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HourlyForecastTypeFullType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HourlyForecastTypeFullTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HourlyForecastTypeFullType {
                date_time_utc: self.date_time_utc,
                condition: self
                    .condition
                    .ok_or_else(|| ErrorKind::MissingElement("condition".into()))?,
                icon_code: self
                    .icon_code
                    .ok_or_else(|| ErrorKind::MissingElement("iconCode".into()))?,
                temperature: self
                    .temperature
                    .ok_or_else(|| ErrorKind::MissingElement("temperature".into()))?,
                lop: self
                    .lop
                    .ok_or_else(|| ErrorKind::MissingElement("lop".into()))?,
                wind_chill: self
                    .wind_chill
                    .ok_or_else(|| ErrorKind::MissingElement("windChill".into()))?,
                humidex: self
                    .humidex
                    .ok_or_else(|| ErrorKind::MissingElement("humidex".into()))?,
                wind: self
                    .wind
                    .ok_or_else(|| ErrorKind::MissingElement("wind".into()))?,
                uv: self.uv,
            })
        }
    }
    #[derive(Debug)]
    pub struct HumidexHourlyTypeDeserializer {
        unit_type: Option<super::ValidUnitTypesType>,
        content: Option<super::ValidHumidexType>,
        state: Box<HumidexHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HumidexHourlyTypeDeserializerState {
        Init__,
        Content__(<super::ValidHumidexType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl HumidexHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                unit_type: unit_type,
                content: None,
                state: Box::new(HumidexHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HumidexHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let HumidexHourlyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidHumidexType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidHumidexType>,
        ) -> DeserializerResult<'de, super::HumidexHourlyType>
        where
            R: DeserializeReader,
        {
            use HumidexHourlyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HumidexHourlyType> for HumidexHourlyTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HumidexHourlyType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HumidexHourlyType>
        where
            R: DeserializeReader,
        {
            use HumidexHourlyTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HumidexHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HumidexHourlyTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HumidexHourlyType {
                unit_type: self.unit_type,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct HumidexTypeDeserializer {
        text_summary: Option<::std::string::String>,
        calculated: Vec<super::CalculatedHumidexType>,
        state: Box<HumidexTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HumidexTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Calculated(Option<<super::CalculatedHumidexType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl HumidexTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                calculated: Vec::new(),
                state: Box::new(HumidexTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HumidexTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HumidexTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::Calculated(Some(deserializer)) => {
                    self.store_calculated(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_calculated(&mut self, value: super::CalculatedHumidexType) -> Result<(), Error> {
            self.calculated.push(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<HumidexTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HumidexTypeDeserializerState::TextSummary(None));
                *self.state = HumidexTypeDeserializerState::Calculated(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = HumidexTypeDeserializerState::Calculated(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(HumidexTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = HumidexTypeDeserializerState::Calculated(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HumidexTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_calculated<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CalculatedHumidexType>,
            fallback: &mut Option<HumidexTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HumidexTypeDeserializerState::Calculated(None));
                *self.state = HumidexTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_calculated(data)?;
                    if self.calculated.len() < 2usize {
                        *self.state = HumidexTypeDeserializerState::Calculated(None);
                    } else {
                        *self.state = HumidexTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(HumidexTypeDeserializerState::Calculated(Some(
                                deserializer,
                            )));
                            *self.state = HumidexTypeDeserializerState::Calculated(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HumidexTypeDeserializerState::Calculated(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HumidexType> for HumidexTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::HumidexType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HumidexType>
        where
            R: DeserializeReader,
        {
            use HumidexTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Calculated(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_calculated(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = HumidexTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Calculated(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"calculated",
                            false,
                        )?;
                        match self.handle_calculated(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HumidexType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, HumidexTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::HumidexType {
                text_summary: self.text_summary,
                calculated: self.calculated,
            })
        }
    }
    #[derive(Debug)]
    pub struct IconCodeHourlyTypeDeserializer {
        format: Option<super::ValidFormatType>,
        content: Option<::std::string::String>,
        state: Box<IconCodeHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IconCodeHourlyTypeDeserializerState {
        Init__,
        Content__(<::std::string::String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IconCodeHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut format: Option<super::ValidFormatType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"format" {
                    reader.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                format: format,
                content: None,
                state: Box::new(IconCodeHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IconCodeHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let IconCodeHourlyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
        ) -> DeserializerResult<'de, super::IconCodeHourlyType>
        where
            R: DeserializeReader,
        {
            use IconCodeHourlyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IconCodeHourlyType> for IconCodeHourlyTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IconCodeHourlyType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IconCodeHourlyType>
        where
            R: DeserializeReader,
        {
            use IconCodeHourlyTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::IconCodeHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                IconCodeHourlyTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::IconCodeHourlyType {
                format: self.format,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IconCodeTypeDeserializer {
        format: super::ValidIconCodesUnitsType,
        content: Option<super::ValidIconCodesType>,
        state: Box<IconCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IconCodeTypeDeserializerState {
        Init__,
        Content__(<super::ValidIconCodesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IconCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut format: Option<super::ValidIconCodesUnitsType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"format" {
                    reader.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                format: format.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("format".into()))
                })?,
                content: None,
                state: Box::new(IconCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IconCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let IconCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidIconCodesType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidIconCodesType>,
        ) -> DeserializerResult<'de, super::IconCodeType>
        where
            R: DeserializeReader,
        {
            use IconCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IconCodeType> for IconCodeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::IconCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IconCodeType>
        where
            R: DeserializeReader,
        {
            use IconCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::IconCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, IconCodeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::IconCodeType {
                format: self.format,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct LocationTypeDeserializer {
        continent: Option<::std::string::String>,
        country: Option<super::CountryType>,
        province: Option<super::ProvinceType>,
        name: Option<super::NameType>,
        region: Option<super::RegionType>,
        state: Box<LocationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LocationTypeDeserializerState {
        Init__,
        Continent(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Country(Option<<super::CountryType as WithDeserializer>::Deserializer>),
        Province(Option<<super::ProvinceType as WithDeserializer>::Deserializer>),
        Name(Option<<super::NameType as WithDeserializer>::Deserializer>),
        Region(Option<<super::RegionType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl LocationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                continent: None,
                country: None,
                province: None,
                name: None,
                region: None,
                state: Box::new(LocationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: LocationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use LocationTypeDeserializerState as S;
            match state {
                S::Continent(Some(deserializer)) => {
                    self.store_continent(deserializer.finish(reader)?)?
                }
                S::Country(Some(deserializer)) => {
                    self.store_country(deserializer.finish(reader)?)?
                }
                S::Province(Some(deserializer)) => {
                    self.store_province(deserializer.finish(reader)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(reader)?)?,
                S::Region(Some(deserializer)) => self.store_region(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_continent(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.continent.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"continent",
                )))?;
            }
            self.continent = Some(value);
            Ok(())
        }
        fn store_country(&mut self, value: super::CountryType) -> Result<(), Error> {
            if self.country.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"country",
                )))?;
            }
            self.country = Some(value);
            Ok(())
        }
        fn store_province(&mut self, value: super::ProvinceType) -> Result<(), Error> {
            if self.province.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"province",
                )))?;
            }
            self.province = Some(value);
            Ok(())
        }
        fn store_name(&mut self, value: super::NameType) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_region(&mut self, value: super::RegionType) -> Result<(), Error> {
            if self.region.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"region",
                )))?;
            }
            self.region = Some(value);
            Ok(())
        }
        fn handle_continent<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<LocationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.continent.is_some() {
                    fallback.get_or_insert(LocationTypeDeserializerState::Continent(None));
                    *self.state = LocationTypeDeserializerState::Country(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = LocationTypeDeserializerState::Continent(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_continent(data)?;
                    *self.state = LocationTypeDeserializerState::Country(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LocationTypeDeserializerState::Continent(Some(
                                deserializer,
                            )));
                            *self.state = LocationTypeDeserializerState::Country(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                LocationTypeDeserializerState::Continent(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_country<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CountryType>,
            fallback: &mut Option<LocationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.country.is_some() {
                    fallback.get_or_insert(LocationTypeDeserializerState::Country(None));
                    *self.state = LocationTypeDeserializerState::Province(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = LocationTypeDeserializerState::Country(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country(data)?;
                    *self.state = LocationTypeDeserializerState::Province(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LocationTypeDeserializerState::Country(Some(
                                deserializer,
                            )));
                            *self.state = LocationTypeDeserializerState::Province(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                LocationTypeDeserializerState::Country(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_province<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ProvinceType>,
            fallback: &mut Option<LocationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.province.is_some() {
                    fallback.get_or_insert(LocationTypeDeserializerState::Province(None));
                    *self.state = LocationTypeDeserializerState::Name(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = LocationTypeDeserializerState::Province(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_province(data)?;
                    *self.state = LocationTypeDeserializerState::Name(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LocationTypeDeserializerState::Province(Some(
                                deserializer,
                            )));
                            *self.state = LocationTypeDeserializerState::Name(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                LocationTypeDeserializerState::Province(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::NameType>,
            fallback: &mut Option<LocationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.name.is_some() {
                    fallback.get_or_insert(LocationTypeDeserializerState::Name(None));
                    *self.state = LocationTypeDeserializerState::Region(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = LocationTypeDeserializerState::Name(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state = LocationTypeDeserializerState::Region(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LocationTypeDeserializerState::Name(Some(
                                deserializer,
                            )));
                            *self.state = LocationTypeDeserializerState::Region(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = LocationTypeDeserializerState::Name(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_region<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RegionType>,
            fallback: &mut Option<LocationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.region.is_some() {
                    fallback.get_or_insert(LocationTypeDeserializerState::Region(None));
                    *self.state = LocationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = LocationTypeDeserializerState::Region(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_region(data)?;
                    *self.state = LocationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LocationTypeDeserializerState::Region(Some(
                                deserializer,
                            )));
                            *self.state = LocationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = LocationTypeDeserializerState::Region(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::LocationType> for LocationTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::LocationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LocationType>
        where
            R: DeserializeReader,
        {
            use LocationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Continent(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_continent(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Country(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_country(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Province(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_province(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Region(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_region(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = LocationTypeDeserializerState::Continent(None);
                        event
                    }
                    (S::Continent(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"continent", false)?;
                        match self.handle_continent(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Country(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"country", false)?;
                        match self.handle_country(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Province(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"province", false)?;
                        match self.handle_province(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"name", false)?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Region(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"region", false)?;
                        match self.handle_region(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::LocationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, LocationTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::LocationType {
                continent: self
                    .continent
                    .ok_or_else(|| ErrorKind::MissingElement("continent".into()))?,
                country: self
                    .country
                    .ok_or_else(|| ErrorKind::MissingElement("country".into()))?,
                province: self
                    .province
                    .ok_or_else(|| ErrorKind::MissingElement("province".into()))?,
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                region: self
                    .region
                    .ok_or_else(|| ErrorKind::MissingElement("region".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct LopHourlyTypeDeserializer {
        category: Option<super::CategoryType>,
        units: Option<super::ValidPopUnitsType>,
        content: Option<super::ValidPopsType>,
        state: Box<LopHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LopHourlyTypeDeserializerState {
        Init__,
        Content__(<super::ValidPopsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl LopHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut category: Option<super::CategoryType> = None;
            let mut units: Option<super::ValidPopUnitsType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"category" {
                    reader.read_attrib(&mut category, b"category", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                category: category,
                units: units,
                content: None,
                state: Box::new(LopHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: LopHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let LopHourlyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidPopsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidPopsType>,
        ) -> DeserializerResult<'de, super::LopHourlyType>
        where
            R: DeserializeReader,
        {
            use LopHourlyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::LopHourlyType> for LopHourlyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::LopHourlyType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LopHourlyType>
        where
            R: DeserializeReader,
        {
            use LopHourlyTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::LopHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, LopHourlyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::LopHourlyType {
                category: self.category,
                units: self.units,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MonthTypeDeserializer {
        name: super::ValidMonthNamesType,
        content: Option<::core::primitive::i32>,
        state: Box<MonthTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MonthTypeDeserializerState {
        Init__,
        Content__(<::core::primitive::i32 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl MonthTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<super::ValidMonthNamesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                content: None,
                state: Box::new(MonthTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MonthTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let MonthTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::core::primitive::i32) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::core::primitive::i32>,
        ) -> DeserializerResult<'de, super::MonthType>
        where
            R: DeserializeReader,
        {
            use MonthTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MonthType> for MonthTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MonthType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MonthType>
        where
            R: DeserializeReader,
        {
            use MonthTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::MonthType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, MonthTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::MonthType {
                name: self.name,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct NameTypeDeserializer {
        code: Option<::std::string::String>,
        lat: Option<::std::string::String>,
        lon: Option<::std::string::String>,
        content: Option<::std::string::String>,
        state: Box<NameTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NameTypeDeserializerState {
        Init__,
        Content__(<::std::string::String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl NameTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut code: Option<::std::string::String> = None;
            let mut lat: Option<::std::string::String> = None;
            let mut lon: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"code" {
                    reader.read_attrib(&mut code, b"code", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"lat" {
                    reader.read_attrib(&mut lat, b"lat", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"lon" {
                    reader.read_attrib(&mut lon, b"lon", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                code: code,
                lat: lat,
                lon: lon,
                content: None,
                state: Box::new(NameTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: NameTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let NameTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
        ) -> DeserializerResult<'de, super::NameType>
        where
            R: DeserializeReader,
        {
            use NameTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::NameType> for NameTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::NameType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NameType>
        where
            R: DeserializeReader,
        {
            use NameTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::NameType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, NameTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::NameType {
                code: self.code,
                lat: self.lat,
                lon: self.lon,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PeriodTypeDeserializer {
        text_forecast_name: Option<super::ValidDayNamesType>,
        content: Option<super::ValidDayNamesType>,
        state: Box<PeriodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PeriodTypeDeserializerState {
        Init__,
        Content__(<super::ValidDayNamesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PeriodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut text_forecast_name: Option<super::ValidDayNamesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"textForecastName" {
                    reader.read_attrib(
                        &mut text_forecast_name,
                        b"textForecastName",
                        &attrib.value,
                    )?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                text_forecast_name: text_forecast_name,
                content: None,
                state: Box::new(PeriodTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PeriodTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PeriodTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidDayNamesType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidDayNamesType>,
        ) -> DeserializerResult<'de, super::PeriodType>
        where
            R: DeserializeReader,
        {
            use PeriodTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PeriodType> for PeriodTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PeriodType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PeriodType>
        where
            R: DeserializeReader,
        {
            use PeriodTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PeriodType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PeriodTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PeriodType {
                text_forecast_name: self.text_forecast_name,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PopTypeDeserializer {
        units: Option<super::ValidPopUnitsType>,
        content: Option<super::ValidPopsType>,
        state: Box<PopTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PopTypeDeserializerState {
        Init__,
        Content__(<super::ValidPopsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PopTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidPopUnitsType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                content: None,
                state: Box::new(PopTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PopTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PopTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidPopsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidPopsType>,
        ) -> DeserializerResult<'de, super::PopType>
        where
            R: DeserializeReader,
        {
            use PopTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PopType> for PopTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PopType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PopType>
        where
            R: DeserializeReader,
        {
            use PopTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PopType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PopTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PopType {
                units: self.units,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PrecipSubTypeForecastTypeDeserializer {
        end: Option<super::ValidPrecipSubTypeForecastHoursType>,
        start: Option<super::ValidPrecipSubTypeForecastHoursType>,
        content: Option<super::ValidPrecipAbbreviatedCodesType>,
        state: Box<PrecipSubTypeForecastTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PrecipSubTypeForecastTypeDeserializerState {
        Init__,
        Content__(<super::ValidPrecipAbbreviatedCodesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PrecipSubTypeForecastTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut end: Option<super::ValidPrecipSubTypeForecastHoursType> = None;
            let mut start: Option<super::ValidPrecipSubTypeForecastHoursType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"end" {
                    reader.read_attrib(&mut end, b"end", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"start" {
                    reader.read_attrib(&mut start, b"start", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                end: end,
                start: start,
                content: None,
                state: Box::new(PrecipSubTypeForecastTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PrecipSubTypeForecastTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PrecipSubTypeForecastTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::ValidPrecipAbbreviatedCodesType,
        ) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidPrecipAbbreviatedCodesType>,
        ) -> DeserializerResult<'de, super::PrecipSubTypeForecastType>
        where
            R: DeserializeReader,
        {
            use PrecipSubTypeForecastTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PrecipSubTypeForecastType>
        for PrecipSubTypeForecastTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PrecipSubTypeForecastType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PrecipSubTypeForecastType>
        where
            R: DeserializeReader,
        {
            use PrecipSubTypeForecastTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PrecipSubTypeForecastType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PrecipSubTypeForecastTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PrecipSubTypeForecastType {
                end: self.end,
                start: self.start,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PrecipTypeDeserializer {
        units: Option<super::ValidPrecipUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        class: Option<super::ValidPrecipClassesType>,
        year: Option<::std::string::String>,
        period: Option<super::PeriodRangeType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidPrecipsType>,
        state: Box<PrecipTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PrecipTypeDeserializerState {
        Init__,
        Content__(<super::ValidPrecipsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PrecipTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidPrecipUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut class: Option<super::ValidPrecipClassesType> = None;
            let mut year: Option<::std::string::String> = None;
            let mut period: Option<super::PeriodRangeType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"class" {
                    reader.read_attrib(&mut class, b"class", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"year" {
                    reader.read_attrib(&mut year, b"year", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"period" {
                    reader.read_attrib(&mut period, b"period", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                class: class,
                year: year,
                period: period,
                qa_value: qa_value,
                content: None,
                state: Box::new(PrecipTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PrecipTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PrecipTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidPrecipsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidPrecipsType>,
        ) -> DeserializerResult<'de, super::PrecipType>
        where
            R: DeserializeReader,
        {
            use PrecipTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PrecipType> for PrecipTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PrecipType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PrecipType>
        where
            R: DeserializeReader,
        {
            use PrecipTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PrecipType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PrecipTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PrecipType {
                units: self.units,
                unit_type: self.unit_type,
                class: self.class,
                year: self.year,
                period: self.period,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PrecipTypeForecastTypeDeserializer {
        text_summary: Option<::std::string::String>,
        precip_type: Vec<super::PrecipSubTypeForecastType>,
        accumulation: Vec<super::AccumulationType>,
        state: Box<PrecipTypeForecastTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PrecipTypeForecastTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        PrecipType(Option<<super::PrecipSubTypeForecastType as WithDeserializer>::Deserializer>),
        Accumulation(Option<<super::AccumulationType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PrecipTypeForecastTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                precip_type: Vec::new(),
                accumulation: Vec::new(),
                state: Box::new(PrecipTypeForecastTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PrecipTypeForecastTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PrecipTypeForecastTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::PrecipType(Some(deserializer)) => {
                    self.store_precip_type(deserializer.finish(reader)?)?
                }
                S::Accumulation(Some(deserializer)) => {
                    self.store_accumulation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_precip_type(
            &mut self,
            value: super::PrecipSubTypeForecastType,
        ) -> Result<(), Error> {
            self.precip_type.push(value);
            Ok(())
        }
        fn store_accumulation(&mut self, value: super::AccumulationType) -> Result<(), Error> {
            self.accumulation.push(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<PrecipTypeForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback
                        .get_or_insert(PrecipTypeForecastTypeDeserializerState::TextSummary(None));
                    *self.state = PrecipTypeForecastTypeDeserializerState::PrecipType(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PrecipTypeForecastTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = PrecipTypeForecastTypeDeserializerState::PrecipType(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PrecipTypeForecastTypeDeserializerState::TextSummary(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = PrecipTypeForecastTypeDeserializerState::PrecipType(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PrecipTypeForecastTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_precip_type<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PrecipSubTypeForecastType>,
            fallback: &mut Option<PrecipTypeForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.precip_type.len() < 1usize {
                    *self.state = PrecipTypeForecastTypeDeserializerState::PrecipType(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback
                        .get_or_insert(PrecipTypeForecastTypeDeserializerState::PrecipType(None));
                    *self.state = PrecipTypeForecastTypeDeserializerState::Accumulation(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_precip_type(data)?;
                    *self.state = PrecipTypeForecastTypeDeserializerState::PrecipType(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PrecipTypeForecastTypeDeserializerState::PrecipType(Some(
                                    deserializer,
                                )),
                            );
                            if self.precip_type.len().saturating_add(1) < 1usize {
                                *self.state =
                                    PrecipTypeForecastTypeDeserializerState::PrecipType(None);
                            } else {
                                *self.state =
                                    PrecipTypeForecastTypeDeserializerState::Accumulation(None);
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PrecipTypeForecastTypeDeserializerState::PrecipType(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_accumulation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AccumulationType>,
            fallback: &mut Option<PrecipTypeForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(PrecipTypeForecastTypeDeserializerState::Accumulation(None));
                *self.state = PrecipTypeForecastTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_accumulation(data)?;
                    if self.accumulation.len() < 4usize {
                        *self.state = PrecipTypeForecastTypeDeserializerState::Accumulation(None);
                    } else {
                        *self.state = PrecipTypeForecastTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PrecipTypeForecastTypeDeserializerState::Accumulation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                PrecipTypeForecastTypeDeserializerState::Accumulation(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PrecipTypeForecastTypeDeserializerState::Accumulation(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PrecipTypeForecastType> for PrecipTypeForecastTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PrecipTypeForecastType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PrecipTypeForecastType>
        where
            R: DeserializeReader,
        {
            use PrecipTypeForecastTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PrecipType(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_precip_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Accumulation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_accumulation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = PrecipTypeForecastTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PrecipType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"precipType",
                            false,
                        )?;
                        match self.handle_precip_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Accumulation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"accumulation",
                            false,
                        )?;
                        match self.handle_accumulation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PrecipTypeForecastType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PrecipTypeForecastTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PrecipTypeForecastType {
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
                precip_type: self.precip_type,
                accumulation: self.accumulation,
            })
        }
    }
    #[derive(Debug)]
    pub struct PressureTypeCondTypeDeserializer {
        units: Option<super::ValidPressureUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        change: Option<super::ValidPressuresType>,
        tendency: Option<super::ValidPressureTendenciesType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidPressuresType>,
        state: Box<PressureTypeCondTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PressureTypeCondTypeDeserializerState {
        Init__,
        Content__(<super::ValidPressuresType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PressureTypeCondTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidPressureUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut change: Option<super::ValidPressuresType> = None;
            let mut tendency: Option<super::ValidPressureTendenciesType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"change" {
                    reader.read_attrib(&mut change, b"change", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"tendency" {
                    reader.read_attrib(&mut tendency, b"tendency", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                change: change,
                tendency: tendency,
                qa_value: qa_value,
                content: None,
                state: Box::new(PressureTypeCondTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PressureTypeCondTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PressureTypeCondTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidPressuresType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidPressuresType>,
        ) -> DeserializerResult<'de, super::PressureTypeCondType>
        where
            R: DeserializeReader,
        {
            use PressureTypeCondTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PressureTypeCondType> for PressureTypeCondTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PressureTypeCondType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PressureTypeCondType>
        where
            R: DeserializeReader,
        {
            use PressureTypeCondTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PressureTypeCondType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PressureTypeCondTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PressureTypeCondType {
                units: self.units,
                unit_type: self.unit_type,
                change: self.change,
                tendency: self.tendency,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PressureTypeForecastTypeDeserializer {
        units: Option<super::ValidPressureUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        content: Option<super::ValidPressuresType>,
        state: Box<PressureTypeForecastTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PressureTypeForecastTypeDeserializerState {
        Init__,
        Content__(<super::ValidPressuresType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PressureTypeForecastTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidPressureUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                content: None,
                state: Box::new(PressureTypeForecastTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PressureTypeForecastTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PressureTypeForecastTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidPressuresType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidPressuresType>,
        ) -> DeserializerResult<'de, super::PressureTypeForecastType>
        where
            R: DeserializeReader,
        {
            use PressureTypeForecastTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PressureTypeForecastType>
        for PressureTypeForecastTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PressureTypeForecastType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PressureTypeForecastType>
        where
            R: DeserializeReader,
        {
            use PressureTypeForecastTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PressureTypeForecastType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PressureTypeForecastTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PressureTypeForecastType {
                units: self.units,
                unit_type: self.unit_type,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PressuresTypeDeserializer {
        text_summary: Option<::std::string::String>,
        pressure: Option<super::PressureTypeForecastType>,
        state: Box<PressuresTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PressuresTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Pressure(Option<<super::PressureTypeForecastType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PressuresTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                pressure: None,
                state: Box::new(PressuresTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PressuresTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PressuresTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::Pressure(Some(deserializer)) => {
                    self.store_pressure(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_pressure(&mut self, value: super::PressureTypeForecastType) -> Result<(), Error> {
            if self.pressure.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"pressure",
                )))?;
            }
            self.pressure = Some(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<PressuresTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(PressuresTypeDeserializerState::TextSummary(None));
                    *self.state = PressuresTypeDeserializerState::Pressure(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PressuresTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = PressuresTypeDeserializerState::Pressure(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(PressuresTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = PressuresTypeDeserializerState::Pressure(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                PressuresTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_pressure<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PressureTypeForecastType>,
            fallback: &mut Option<PressuresTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.pressure.is_some() {
                    fallback.get_or_insert(PressuresTypeDeserializerState::Pressure(None));
                    *self.state = PressuresTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PressuresTypeDeserializerState::Pressure(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pressure(data)?;
                    *self.state = PressuresTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(PressuresTypeDeserializerState::Pressure(Some(
                                deserializer,
                            )));
                            *self.state = PressuresTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                PressuresTypeDeserializerState::Pressure(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PressuresType> for PressuresTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PressuresType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PressuresType>
        where
            R: DeserializeReader,
        {
            use PressuresTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Pressure(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pressure(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = PressuresTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Pressure(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"pressure", false)?;
                        match self.handle_pressure(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PressuresType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PressuresTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PressuresType {
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
                pressure: self
                    .pressure
                    .ok_or_else(|| ErrorKind::MissingElement("pressure".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ProvinceTypeDeserializer {
        code: Option<::std::string::String>,
        content: Option<::std::string::String>,
        state: Box<ProvinceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ProvinceTypeDeserializerState {
        Init__,
        Content__(<::std::string::String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ProvinceTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut code: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"code" {
                    reader.read_attrib(&mut code, b"code", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                code: code,
                content: None,
                state: Box::new(ProvinceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ProvinceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ProvinceTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
        ) -> DeserializerResult<'de, super::ProvinceType>
        where
            R: DeserializeReader,
        {
            use ProvinceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ProvinceType> for ProvinceTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ProvinceType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ProvinceType>
        where
            R: DeserializeReader,
        {
            use ProvinceTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ProvinceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ProvinceTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ProvinceType {
                code: self.code,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RegionTypeDeserializer {
        code: Option<::std::string::String>,
        content: Option<::std::string::String>,
        state: Box<RegionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RegionTypeDeserializerState {
        Init__,
        Content__(<::std::string::String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RegionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut code: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"code" {
                    reader.read_attrib(&mut code, b"code", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                code: code,
                content: None,
                state: Box::new(RegionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RegionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RegionTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
        ) -> DeserializerResult<'de, super::RegionType>
        where
            R: DeserializeReader,
        {
            use RegionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RegionType> for RegionTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RegionType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RegionType>
        where
            R: DeserializeReader,
        {
            use RegionTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RegionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, RegionTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::RegionType {
                code: self.code,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RegionalNormalsTypeDeserializer {
        text_summary: Option<::std::string::String>,
        temperature: Vec<super::TemperatureType>,
        state: Box<RegionalNormalsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RegionalNormalsTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Temperature(Option<<super::TemperatureType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RegionalNormalsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                temperature: Vec::new(),
                state: Box::new(RegionalNormalsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RegionalNormalsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RegionalNormalsTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::Temperature(Some(deserializer)) => {
                    self.store_temperature(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_temperature(&mut self, value: super::TemperatureType) -> Result<(), Error> {
            self.temperature.push(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<RegionalNormalsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(RegionalNormalsTypeDeserializerState::TextSummary(None));
                *self.state = RegionalNormalsTypeDeserializerState::Temperature(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = RegionalNormalsTypeDeserializerState::Temperature(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RegionalNormalsTypeDeserializerState::TextSummary(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = RegionalNormalsTypeDeserializerState::Temperature(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = RegionalNormalsTypeDeserializerState::TextSummary(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_temperature<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TemperatureType>,
            fallback: &mut Option<RegionalNormalsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(RegionalNormalsTypeDeserializerState::Temperature(None));
                *self.state = RegionalNormalsTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_temperature(data)?;
                    if self.temperature.len() < 2usize {
                        *self.state = RegionalNormalsTypeDeserializerState::Temperature(None);
                    } else {
                        *self.state = RegionalNormalsTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RegionalNormalsTypeDeserializerState::Temperature(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = RegionalNormalsTypeDeserializerState::Temperature(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = RegionalNormalsTypeDeserializerState::Temperature(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RegionalNormalsType> for RegionalNormalsTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RegionalNormalsType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RegionalNormalsType>
        where
            R: DeserializeReader,
        {
            use RegionalNormalsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = RegionalNormalsTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"temperature",
                            false,
                        )?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RegionalNormalsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RegionalNormalsTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RegionalNormalsType {
                text_summary: self.text_summary,
                temperature: self.temperature,
            })
        }
    }
    #[derive(Debug)]
    pub struct RelativeHumidityTypeDeserializer {
        units: Option<super::ValidRelativeHumidityUnitsType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidRelativeHumiditiesType>,
        state: Box<RelativeHumidityTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RelativeHumidityTypeDeserializerState {
        Init__,
        Content__(<super::ValidRelativeHumiditiesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RelativeHumidityTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidRelativeHumidityUnitsType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                qa_value: qa_value,
                content: None,
                state: Box::new(RelativeHumidityTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RelativeHumidityTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RelativeHumidityTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::ValidRelativeHumiditiesType,
        ) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidRelativeHumiditiesType>,
        ) -> DeserializerResult<'de, super::RelativeHumidityType>
        where
            R: DeserializeReader,
        {
            use RelativeHumidityTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RelativeHumidityType> for RelativeHumidityTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RelativeHumidityType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RelativeHumidityType>
        where
            R: DeserializeReader,
        {
            use RelativeHumidityTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RelativeHumidityType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RelativeHumidityTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RelativeHumidityType {
                units: self.units,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RiseSetElementTypeDeserializer {
        time: Option<super::TimeType>,
        state: Box<RiseSetElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RiseSetElementTypeDeserializerState {
        Init__,
        Time(Option<<super::TimeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RiseSetElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                time: None,
                state: Box::new(RiseSetElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RiseSetElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RiseSetElementTypeDeserializerState as S;
            match state {
                S::Time(Some(deserializer)) => self.store_time(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_time(&mut self, value: super::TimeType) -> Result<(), Error> {
            if self.time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"time")))?;
            }
            self.time = Some(value);
            Ok(())
        }
        fn handle_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TimeType>,
            fallback: &mut Option<RiseSetElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.time.is_some() {
                    fallback.get_or_insert(RiseSetElementTypeDeserializerState::Time(None));
                    *self.state = RiseSetElementTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = RiseSetElementTypeDeserializerState::Time(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_time(data)?;
                    *self.state = RiseSetElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RiseSetElementTypeDeserializerState::Time(
                                Some(deserializer),
                            ));
                            *self.state = RiseSetElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RiseSetElementTypeDeserializerState::Time(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RiseSetElementType> for RiseSetElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RiseSetElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RiseSetElementType>
        where
            R: DeserializeReader,
        {
            use RiseSetElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Time(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = RiseSetElementTypeDeserializerState::Time(None);
                        event
                    }
                    (S::Time(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"time", false)?;
                        match self.handle_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RiseSetElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RiseSetElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RiseSetElementType {
                time: self
                    .time
                    .ok_or_else(|| ErrorKind::MissingElement("time".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RiseSetTypeDeserializer {
        disclaimer: Option<::std::string::String>,
        date_time: Vec<super::DateStampType>,
        state: Box<RiseSetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RiseSetTypeDeserializerState {
        Init__,
        Disclaimer(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        DateTime(Option<<super::DateStampType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RiseSetTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                disclaimer: None,
                date_time: Vec::new(),
                state: Box::new(RiseSetTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RiseSetTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RiseSetTypeDeserializerState as S;
            match state {
                S::Disclaimer(Some(deserializer)) => {
                    self.store_disclaimer(deserializer.finish(reader)?)?
                }
                S::DateTime(Some(deserializer)) => {
                    self.store_date_time(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_disclaimer(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.disclaimer.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"disclaimer",
                )))?;
            }
            self.disclaimer = Some(value);
            Ok(())
        }
        fn store_date_time(&mut self, value: super::DateStampType) -> Result<(), Error> {
            self.date_time.push(value);
            Ok(())
        }
        fn handle_disclaimer<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<RiseSetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(RiseSetTypeDeserializerState::Disclaimer(None));
                *self.state = RiseSetTypeDeserializerState::DateTime(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_disclaimer(data)?;
                    *self.state = RiseSetTypeDeserializerState::DateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RiseSetTypeDeserializerState::Disclaimer(Some(
                                deserializer,
                            )));
                            *self.state = RiseSetTypeDeserializerState::DateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RiseSetTypeDeserializerState::Disclaimer(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateStampType>,
            fallback: &mut Option<RiseSetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(RiseSetTypeDeserializerState::DateTime(None));
                *self.state = RiseSetTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time(data)?;
                    if self.date_time.len() < 12usize {
                        *self.state = RiseSetTypeDeserializerState::DateTime(None);
                    } else {
                        *self.state = RiseSetTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RiseSetTypeDeserializerState::DateTime(Some(
                                deserializer,
                            )));
                            *self.state = RiseSetTypeDeserializerState::DateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RiseSetTypeDeserializerState::DateTime(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RiseSetType> for RiseSetTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RiseSetType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RiseSetType>
        where
            R: DeserializeReader,
        {
            use RiseSetTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Disclaimer(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_disclaimer(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = RiseSetTypeDeserializerState::Disclaimer(None);
                        event
                    }
                    (S::Disclaimer(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"disclaimer",
                            false,
                        )?;
                        match self.handle_disclaimer(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dateTime", false)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RiseSetType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, RiseSetTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::RiseSetType {
                disclaimer: self.disclaimer,
                date_time: self.date_time,
            })
        }
    }
    #[derive(Debug)]
    pub struct SiteDataElementTypeDeserializer {
        license: Option<::std::string::String>,
        date_time: Vec<super::DateStampType>,
        location: Option<super::LocationType>,
        warnings: Option<super::WarningsType>,
        current_conditions: Option<super::CurrentConditionsType>,
        forecast_group: Option<super::ForecastGroupTypeFullType>,
        hourly_forecast_group: Option<super::HourlyForecastGroupTypeFullType>,
        rise_set: Option<super::RiseSetType>,
        state: Box<SiteDataElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SiteDataElementTypeDeserializerState {
        Init__,
        License(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        DateTime(Option<<super::DateStampType as WithDeserializer>::Deserializer>),
        Location(Option<<super::LocationType as WithDeserializer>::Deserializer>),
        Warnings(Option<<super::WarningsType as WithDeserializer>::Deserializer>),
        CurrentConditions(Option<<super::CurrentConditionsType as WithDeserializer>::Deserializer>),
        ForecastGroup(Option<<super::ForecastGroupTypeFullType as WithDeserializer>::Deserializer>),
        HourlyForecastGroup(
            Option<<super::HourlyForecastGroupTypeFullType as WithDeserializer>::Deserializer>,
        ),
        RiseSet(Option<<super::RiseSetType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SiteDataElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                license: None,
                date_time: Vec::new(),
                location: None,
                warnings: None,
                current_conditions: None,
                forecast_group: None,
                hourly_forecast_group: None,
                rise_set: None,
                state: Box::new(SiteDataElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SiteDataElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SiteDataElementTypeDeserializerState as S;
            match state {
                S::License(Some(deserializer)) => {
                    self.store_license(deserializer.finish(reader)?)?
                }
                S::DateTime(Some(deserializer)) => {
                    self.store_date_time(deserializer.finish(reader)?)?
                }
                S::Location(Some(deserializer)) => {
                    self.store_location(deserializer.finish(reader)?)?
                }
                S::Warnings(Some(deserializer)) => {
                    self.store_warnings(deserializer.finish(reader)?)?
                }
                S::CurrentConditions(Some(deserializer)) => {
                    self.store_current_conditions(deserializer.finish(reader)?)?
                }
                S::ForecastGroup(Some(deserializer)) => {
                    self.store_forecast_group(deserializer.finish(reader)?)?
                }
                S::HourlyForecastGroup(Some(deserializer)) => {
                    self.store_hourly_forecast_group(deserializer.finish(reader)?)?
                }
                S::RiseSet(Some(deserializer)) => {
                    self.store_rise_set(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_license(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.license.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"license",
                )))?;
            }
            self.license = Some(value);
            Ok(())
        }
        fn store_date_time(&mut self, value: super::DateStampType) -> Result<(), Error> {
            self.date_time.push(value);
            Ok(())
        }
        fn store_location(&mut self, value: super::LocationType) -> Result<(), Error> {
            if self.location.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"location",
                )))?;
            }
            self.location = Some(value);
            Ok(())
        }
        fn store_warnings(&mut self, value: super::WarningsType) -> Result<(), Error> {
            if self.warnings.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"warnings",
                )))?;
            }
            self.warnings = Some(value);
            Ok(())
        }
        fn store_current_conditions(
            &mut self,
            value: super::CurrentConditionsType,
        ) -> Result<(), Error> {
            if self.current_conditions.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"currentConditions",
                )))?;
            }
            self.current_conditions = Some(value);
            Ok(())
        }
        fn store_forecast_group(
            &mut self,
            value: super::ForecastGroupTypeFullType,
        ) -> Result<(), Error> {
            if self.forecast_group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"forecastGroup",
                )))?;
            }
            self.forecast_group = Some(value);
            Ok(())
        }
        fn store_hourly_forecast_group(
            &mut self,
            value: super::HourlyForecastGroupTypeFullType,
        ) -> Result<(), Error> {
            if self.hourly_forecast_group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"hourlyForecastGroup",
                )))?;
            }
            self.hourly_forecast_group = Some(value);
            Ok(())
        }
        fn store_rise_set(&mut self, value: super::RiseSetType) -> Result<(), Error> {
            if self.rise_set.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"riseSet",
                )))?;
            }
            self.rise_set = Some(value);
            Ok(())
        }
        fn handle_license<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.license.is_some() {
                    fallback.get_or_insert(SiteDataElementTypeDeserializerState::License(None));
                    *self.state = SiteDataElementTypeDeserializerState::DateTime(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::License(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_license(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::DateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteDataElementTypeDeserializerState::License(
                                Some(deserializer),
                            ));
                            *self.state = SiteDataElementTypeDeserializerState::DateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteDataElementTypeDeserializerState::License(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateStampType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.date_time.len() < 2usize {
                    *self.state = SiteDataElementTypeDeserializerState::DateTime(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(SiteDataElementTypeDeserializerState::DateTime(None));
                    *self.state = SiteDataElementTypeDeserializerState::Location(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time(data)?;
                    if self.date_time.len() < 2usize {
                        *self.state = SiteDataElementTypeDeserializerState::DateTime(None);
                    } else {
                        *self.state = SiteDataElementTypeDeserializerState::Location(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteDataElementTypeDeserializerState::DateTime(
                                Some(deserializer),
                            ));
                            if self.date_time.len().saturating_add(1) < 2usize {
                                *self.state = SiteDataElementTypeDeserializerState::DateTime(None);
                            } else {
                                *self.state = SiteDataElementTypeDeserializerState::Location(None);
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteDataElementTypeDeserializerState::DateTime(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_location<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::LocationType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.location.is_some() {
                    fallback.get_or_insert(SiteDataElementTypeDeserializerState::Location(None));
                    *self.state = SiteDataElementTypeDeserializerState::Warnings(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::Location(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_location(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::Warnings(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteDataElementTypeDeserializerState::Location(
                                Some(deserializer),
                            ));
                            *self.state = SiteDataElementTypeDeserializerState::Warnings(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteDataElementTypeDeserializerState::Location(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_warnings<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WarningsType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.warnings.is_some() {
                    fallback.get_or_insert(SiteDataElementTypeDeserializerState::Warnings(None));
                    *self.state = SiteDataElementTypeDeserializerState::CurrentConditions(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::Warnings(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_warnings(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::CurrentConditions(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteDataElementTypeDeserializerState::Warnings(
                                Some(deserializer),
                            ));
                            *self.state =
                                SiteDataElementTypeDeserializerState::CurrentConditions(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteDataElementTypeDeserializerState::Warnings(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_current_conditions<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CurrentConditionsType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.current_conditions.is_some() {
                    fallback.get_or_insert(
                        SiteDataElementTypeDeserializerState::CurrentConditions(None),
                    );
                    *self.state = SiteDataElementTypeDeserializerState::ForecastGroup(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::CurrentConditions(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_current_conditions(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::ForecastGroup(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SiteDataElementTypeDeserializerState::CurrentConditions(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SiteDataElementTypeDeserializerState::ForecastGroup(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SiteDataElementTypeDeserializerState::CurrentConditions(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_forecast_group<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ForecastGroupTypeFullType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.forecast_group.is_some() {
                    fallback
                        .get_or_insert(SiteDataElementTypeDeserializerState::ForecastGroup(None));
                    *self.state = SiteDataElementTypeDeserializerState::HourlyForecastGroup(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::ForecastGroup(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_forecast_group(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::HourlyForecastGroup(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SiteDataElementTypeDeserializerState::ForecastGroup(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                SiteDataElementTypeDeserializerState::HourlyForecastGroup(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SiteDataElementTypeDeserializerState::ForecastGroup(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_hourly_forecast_group<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HourlyForecastGroupTypeFullType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.hourly_forecast_group.is_some() {
                    fallback.get_or_insert(
                        SiteDataElementTypeDeserializerState::HourlyForecastGroup(None),
                    );
                    *self.state = SiteDataElementTypeDeserializerState::RiseSet(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::HourlyForecastGroup(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hourly_forecast_group(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::RiseSet(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SiteDataElementTypeDeserializerState::HourlyForecastGroup(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SiteDataElementTypeDeserializerState::RiseSet(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SiteDataElementTypeDeserializerState::HourlyForecastGroup(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_rise_set<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RiseSetType>,
            fallback: &mut Option<SiteDataElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.rise_set.is_some() {
                    fallback.get_or_insert(SiteDataElementTypeDeserializerState::RiseSet(None));
                    *self.state = SiteDataElementTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteDataElementTypeDeserializerState::RiseSet(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_rise_set(data)?;
                    *self.state = SiteDataElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteDataElementTypeDeserializerState::RiseSet(
                                Some(deserializer),
                            ));
                            *self.state = SiteDataElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteDataElementTypeDeserializerState::RiseSet(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SiteDataElementType> for SiteDataElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SiteDataElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SiteDataElementType>
        where
            R: DeserializeReader,
        {
            use SiteDataElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::License(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_license(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Location(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_location(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Warnings(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_warnings(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CurrentConditions(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_current_conditions(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ForecastGroup(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_forecast_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::HourlyForecastGroup(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_hourly_forecast_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RiseSet(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_rise_set(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = SiteDataElementTypeDeserializerState::License(None);
                        event
                    }
                    (S::License(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"license", false)?;
                        match self.handle_license(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dateTime", false)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Location(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"location", false)?;
                        match self.handle_location(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Warnings(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"warnings", false)?;
                        match self.handle_warnings(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CurrentConditions(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"currentConditions",
                            false,
                        )?;
                        match self.handle_current_conditions(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ForecastGroup(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"forecastGroup",
                            false,
                        )?;
                        match self.handle_forecast_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::HourlyForecastGroup(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"hourlyForecastGroup",
                            false,
                        )?;
                        match self.handle_hourly_forecast_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RiseSet(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"riseSet", false)?;
                        match self.handle_rise_set(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SiteDataElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SiteDataElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SiteDataElementType {
                license: self
                    .license
                    .ok_or_else(|| ErrorKind::MissingElement("license".into()))?,
                date_time: self.date_time.try_into().map_err(|vec: Vec<_>| {
                    ErrorKind::InsufficientSize {
                        min: 2usize,
                        max: 2usize,
                        actual: vec.len(),
                    }
                })?,
                location: self
                    .location
                    .ok_or_else(|| ErrorKind::MissingElement("location".into()))?,
                warnings: self
                    .warnings
                    .ok_or_else(|| ErrorKind::MissingElement("warnings".into()))?,
                current_conditions: self
                    .current_conditions
                    .ok_or_else(|| ErrorKind::MissingElement("currentConditions".into()))?,
                forecast_group: self
                    .forecast_group
                    .ok_or_else(|| ErrorKind::MissingElement("forecastGroup".into()))?,
                hourly_forecast_group: self
                    .hourly_forecast_group
                    .ok_or_else(|| ErrorKind::MissingElement("hourlyForecastGroup".into()))?,
                rise_set: self
                    .rise_set
                    .ok_or_else(|| ErrorKind::MissingElement("riseSet".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SiteListElementTypeDeserializer {
        site: Vec<super::SiteType>,
        state: Box<SiteListElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SiteListElementTypeDeserializerState {
        Init__,
        Site(Option<<super::SiteType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SiteListElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                site: Vec::new(),
                state: Box::new(SiteListElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SiteListElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SiteListElementTypeDeserializerState as S;
            match state {
                S::Site(Some(deserializer)) => self.store_site(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_site(&mut self, value: super::SiteType) -> Result<(), Error> {
            self.site.push(value);
            Ok(())
        }
        fn handle_site<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SiteType>,
            fallback: &mut Option<SiteListElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(SiteListElementTypeDeserializerState::Site(None));
                *self.state = SiteListElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_site(data)?;
                    *self.state = SiteListElementTypeDeserializerState::Site(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteListElementTypeDeserializerState::Site(
                                Some(deserializer),
                            ));
                            *self.state = SiteListElementTypeDeserializerState::Site(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteListElementTypeDeserializerState::Site(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SiteListElementType> for SiteListElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SiteListElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SiteListElementType>
        where
            R: DeserializeReader,
        {
            use SiteListElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Site(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_site(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = SiteListElementTypeDeserializerState::Site(None);
                        event
                    }
                    (S::Site(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"site", false)?;
                        match self.handle_site(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SiteListElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SiteListElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SiteListElementType { site: self.site })
        }
    }
    #[derive(Debug)]
    pub struct SiteTypeDeserializer {
        code: Option<::std::string::String>,
        name_en: Option<::std::string::String>,
        name_fr: Option<::std::string::String>,
        province_code: Option<::std::string::String>,
        state: Box<SiteTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SiteTypeDeserializerState {
        Init__,
        NameEn(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        NameFr(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        ProvinceCode(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SiteTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut code: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"code" {
                    reader.read_attrib(&mut code, b"code", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                code: code,
                name_en: None,
                name_fr: None,
                province_code: None,
                state: Box::new(SiteTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SiteTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SiteTypeDeserializerState as S;
            match state {
                S::NameEn(Some(deserializer)) => {
                    self.store_name_en(deserializer.finish(reader)?)?
                }
                S::NameFr(Some(deserializer)) => {
                    self.store_name_fr(deserializer.finish(reader)?)?
                }
                S::ProvinceCode(Some(deserializer)) => {
                    self.store_province_code(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_name_en(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.name_en.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"nameEn",
                )))?;
            }
            self.name_en = Some(value);
            Ok(())
        }
        fn store_name_fr(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.name_fr.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"nameFr",
                )))?;
            }
            self.name_fr = Some(value);
            Ok(())
        }
        fn store_province_code(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.province_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"provinceCode",
                )))?;
            }
            self.province_code = Some(value);
            Ok(())
        }
        fn handle_name_en<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<SiteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.name_en.is_some() {
                    fallback.get_or_insert(SiteTypeDeserializerState::NameEn(None));
                    *self.state = SiteTypeDeserializerState::NameFr(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteTypeDeserializerState::NameEn(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name_en(data)?;
                    *self.state = SiteTypeDeserializerState::NameFr(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteTypeDeserializerState::NameEn(Some(
                                deserializer,
                            )));
                            *self.state = SiteTypeDeserializerState::NameFr(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SiteTypeDeserializerState::NameEn(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_name_fr<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<SiteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.name_fr.is_some() {
                    fallback.get_or_insert(SiteTypeDeserializerState::NameFr(None));
                    *self.state = SiteTypeDeserializerState::ProvinceCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteTypeDeserializerState::NameFr(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name_fr(data)?;
                    *self.state = SiteTypeDeserializerState::ProvinceCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteTypeDeserializerState::NameFr(Some(
                                deserializer,
                            )));
                            *self.state = SiteTypeDeserializerState::ProvinceCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SiteTypeDeserializerState::NameFr(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_province_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<SiteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.province_code.is_some() {
                    fallback.get_or_insert(SiteTypeDeserializerState::ProvinceCode(None));
                    *self.state = SiteTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SiteTypeDeserializerState::ProvinceCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_province_code(data)?;
                    *self.state = SiteTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SiteTypeDeserializerState::ProvinceCode(Some(
                                deserializer,
                            )));
                            *self.state = SiteTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SiteTypeDeserializerState::ProvinceCode(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SiteType> for SiteTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SiteType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SiteType>
        where
            R: DeserializeReader,
        {
            use SiteTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::NameEn(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name_en(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::NameFr(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name_fr(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProvinceCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_province_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = SiteTypeDeserializerState::NameEn(None);
                        event
                    }
                    (S::NameEn(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"nameEn", false)?;
                        match self.handle_name_en(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::NameFr(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"nameFr", false)?;
                        match self.handle_name_fr(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProvinceCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"provinceCode",
                            false,
                        )?;
                        match self.handle_province_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SiteType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SiteTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SiteType {
                code: self.code,
                name_en: self
                    .name_en
                    .ok_or_else(|| ErrorKind::MissingElement("nameEn".into()))?,
                name_fr: self
                    .name_fr
                    .ok_or_else(|| ErrorKind::MissingElement("nameFr".into()))?,
                province_code: self
                    .province_code
                    .ok_or_else(|| ErrorKind::MissingElement("provinceCode".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SnowLevelTypeDeserializer {
        text_summary: Option<::std::string::String>,
        state: Box<SnowLevelTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SnowLevelTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SnowLevelTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                state: Box::new(SnowLevelTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SnowLevelTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SnowLevelTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<SnowLevelTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(SnowLevelTypeDeserializerState::TextSummary(None));
                    *self.state = SnowLevelTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SnowLevelTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = SnowLevelTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SnowLevelTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = SnowLevelTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SnowLevelTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SnowLevelType> for SnowLevelTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SnowLevelType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SnowLevelType>
        where
            R: DeserializeReader,
        {
            use SnowLevelTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = SnowLevelTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SnowLevelType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SnowLevelTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SnowLevelType {
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct StationTypeDeserializer {
        code: Option<::std::string::String>,
        lat: Option<super::ValidLatLonType>,
        lon: Option<super::ValidLatLonType>,
        country: Option<super::ValidCountryCodeType>,
        content: Option<::std::string::String>,
        state: Box<StationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum StationTypeDeserializerState {
        Init__,
        Content__(<::std::string::String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl StationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut code: Option<::std::string::String> = None;
            let mut lat: Option<super::ValidLatLonType> = None;
            let mut lon: Option<super::ValidLatLonType> = None;
            let mut country: Option<super::ValidCountryCodeType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"code" {
                    reader.read_attrib(&mut code, b"code", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"lat" {
                    reader.read_attrib(&mut lat, b"lat", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"lon" {
                    reader.read_attrib(&mut lon, b"lon", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"country" {
                    reader.read_attrib(&mut country, b"country", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                code: code,
                lat: lat,
                lon: lon,
                country: country,
                content: None,
                state: Box::new(StationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: StationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let StationTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
        ) -> DeserializerResult<'de, super::StationType>
        where
            R: DeserializeReader,
        {
            use StationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::StationType> for StationTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::StationType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StationType>
        where
            R: DeserializeReader,
        {
            use StationTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::StationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, StationTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::StationType {
                code: self.code,
                lat: self.lat,
                lon: self.lon,
                country: self.country,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TemperatureHourlyTypeDeserializer {
        units: Option<super::ValidTemperatureUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        content: Option<super::ValidTemperaturesType>,
        state: Box<TemperatureHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TemperatureHourlyTypeDeserializerState {
        Init__,
        Content__(<super::ValidTemperaturesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TemperatureHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidTemperatureUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                content: None,
                state: Box::new(TemperatureHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TemperatureHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TemperatureHourlyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidTemperaturesType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidTemperaturesType>,
        ) -> DeserializerResult<'de, super::TemperatureHourlyType>
        where
            R: DeserializeReader,
        {
            use TemperatureHourlyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TemperatureHourlyType> for TemperatureHourlyTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TemperatureHourlyType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TemperatureHourlyType>
        where
            R: DeserializeReader,
        {
            use TemperatureHourlyTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TemperatureHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TemperatureHourlyTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TemperatureHourlyType {
                units: self.units,
                unit_type: self.unit_type,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TemperatureTypeDeserializer {
        units: Option<super::ValidTemperatureUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        class: Option<super::ValidTemperatureClassesType>,
        year: Option<::std::string::String>,
        period: Option<super::PeriodRangeType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidTemperaturesType>,
        state: Box<TemperatureTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TemperatureTypeDeserializerState {
        Init__,
        Content__(<super::ValidTemperaturesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TemperatureTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidTemperatureUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut class: Option<super::ValidTemperatureClassesType> = None;
            let mut year: Option<::std::string::String> = None;
            let mut period: Option<super::PeriodRangeType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"class" {
                    reader.read_attrib(&mut class, b"class", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"year" {
                    reader.read_attrib(&mut year, b"year", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"period" {
                    reader.read_attrib(&mut period, b"period", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                class: class,
                year: year,
                period: period,
                qa_value: qa_value,
                content: None,
                state: Box::new(TemperatureTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TemperatureTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TemperatureTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidTemperaturesType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidTemperaturesType>,
        ) -> DeserializerResult<'de, super::TemperatureType>
        where
            R: DeserializeReader,
        {
            use TemperatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TemperatureType> for TemperatureTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TemperatureType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TemperatureType>
        where
            R: DeserializeReader,
        {
            use TemperatureTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TemperatureType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TemperatureTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TemperatureType {
                units: self.units,
                unit_type: self.unit_type,
                class: self.class,
                year: self.year,
                period: self.period,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TemperaturesTypeDeserializer {
        text_summary: Option<::std::string::String>,
        temperature: Vec<super::TemperatureType>,
        state: Box<TemperaturesTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TemperaturesTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Temperature(Option<<super::TemperatureType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TemperaturesTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                temperature: Vec::new(),
                state: Box::new(TemperaturesTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TemperaturesTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TemperaturesTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::Temperature(Some(deserializer)) => {
                    self.store_temperature(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_temperature(&mut self, value: super::TemperatureType) -> Result<(), Error> {
            self.temperature.push(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<TemperaturesTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(TemperaturesTypeDeserializerState::TextSummary(None));
                    *self.state = TemperaturesTypeDeserializerState::Temperature(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TemperaturesTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = TemperaturesTypeDeserializerState::Temperature(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TemperaturesTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = TemperaturesTypeDeserializerState::Temperature(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TemperaturesTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_temperature<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TemperatureType>,
            fallback: &mut Option<TemperaturesTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.temperature.len() < 1usize {
                    *self.state = TemperaturesTypeDeserializerState::Temperature(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(TemperaturesTypeDeserializerState::Temperature(None));
                    *self.state = TemperaturesTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_temperature(data)?;
                    if self.temperature.len() < 2usize {
                        *self.state = TemperaturesTypeDeserializerState::Temperature(None);
                    } else {
                        *self.state = TemperaturesTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TemperaturesTypeDeserializerState::Temperature(
                                Some(deserializer),
                            ));
                            if self.temperature.len().saturating_add(1) < 1usize {
                                *self.state = TemperaturesTypeDeserializerState::Temperature(None);
                            } else {
                                *self.state = TemperaturesTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TemperaturesTypeDeserializerState::Temperature(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TemperaturesType> for TemperaturesTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TemperaturesType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TemperaturesType>
        where
            R: DeserializeReader,
        {
            use TemperaturesTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TemperaturesTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Temperature(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"temperature",
                            false,
                        )?;
                        match self.handle_temperature(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TemperaturesType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TemperaturesTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TemperaturesType {
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
                temperature: self.temperature,
            })
        }
    }
    #[derive(Debug)]
    pub struct TimeTypeDeserializer {
        name: Option<::std::string::String>,
        zone: Option<super::ValidTimeZonesType>,
        hour: Option<super::HourType>,
        minute: Option<super::MinuteType>,
        second: Option<super::SecondType>,
        state: Box<TimeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TimeTypeDeserializerState {
        Init__,
        Hour(Option<<super::HourType as WithDeserializer>::Deserializer>),
        Minute(Option<<super::MinuteType as WithDeserializer>::Deserializer>),
        Second(Option<<super::SecondType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TimeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<::std::string::String> = None;
            let mut zone: Option<super::ValidTimeZonesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"zone" {
                    reader.read_attrib(&mut zone, b"zone", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name,
                zone: zone,
                hour: None,
                minute: None,
                second: None,
                state: Box::new(TimeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TimeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TimeTypeDeserializerState as S;
            match state {
                S::Hour(Some(deserializer)) => self.store_hour(deserializer.finish(reader)?)?,
                S::Minute(Some(deserializer)) => self.store_minute(deserializer.finish(reader)?)?,
                S::Second(Some(deserializer)) => self.store_second(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_hour(&mut self, value: super::HourType) -> Result<(), Error> {
            if self.hour.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"hour")))?;
            }
            self.hour = Some(value);
            Ok(())
        }
        fn store_minute(&mut self, value: super::MinuteType) -> Result<(), Error> {
            if self.minute.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"minute",
                )))?;
            }
            self.minute = Some(value);
            Ok(())
        }
        fn store_second(&mut self, value: super::SecondType) -> Result<(), Error> {
            if self.second.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"second",
                )))?;
            }
            self.second = Some(value);
            Ok(())
        }
        fn handle_hour<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HourType>,
            fallback: &mut Option<TimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.hour.is_some() {
                    fallback.get_or_insert(TimeTypeDeserializerState::Hour(None));
                    *self.state = TimeTypeDeserializerState::Minute(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TimeTypeDeserializerState::Hour(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hour(data)?;
                    *self.state = TimeTypeDeserializerState::Minute(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(TimeTypeDeserializerState::Hour(Some(deserializer)));
                            *self.state = TimeTypeDeserializerState::Minute(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TimeTypeDeserializerState::Hour(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_minute<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MinuteType>,
            fallback: &mut Option<TimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.minute.is_some() {
                    fallback.get_or_insert(TimeTypeDeserializerState::Minute(None));
                    *self.state = TimeTypeDeserializerState::Second(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TimeTypeDeserializerState::Minute(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_minute(data)?;
                    *self.state = TimeTypeDeserializerState::Second(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TimeTypeDeserializerState::Minute(Some(
                                deserializer,
                            )));
                            *self.state = TimeTypeDeserializerState::Second(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TimeTypeDeserializerState::Minute(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_second<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SecondType>,
            fallback: &mut Option<TimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TimeTypeDeserializerState::Second(None));
                *self.state = TimeTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_second(data)?;
                    *self.state = TimeTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TimeTypeDeserializerState::Second(Some(
                                deserializer,
                            )));
                            *self.state = TimeTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TimeTypeDeserializerState::Second(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TimeType> for TimeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TimeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TimeType>
        where
            R: DeserializeReader,
        {
            use TimeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Hour(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_hour(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Minute(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_minute(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Second(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_second(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TimeTypeDeserializerState::Hour(None);
                        event
                    }
                    (S::Hour(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"hour", false)?;
                        match self.handle_hour(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Minute(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"minute", false)?;
                        match self.handle_minute(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Second(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"second", false)?;
                        match self.handle_second(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TimeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TimeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TimeType {
                name: self.name,
                zone: self.zone,
                hour: self
                    .hour
                    .ok_or_else(|| ErrorKind::MissingElement("hour".into()))?,
                minute: self
                    .minute
                    .ok_or_else(|| ErrorKind::MissingElement("minute".into()))?,
                second: self.second,
            })
        }
    }
    #[derive(Debug)]
    pub struct UvHourlyTypeDeserializer {
        index: Option<super::UvIndexType>,
        state: Box<UvHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UvHourlyTypeDeserializerState {
        Init__,
        Index(Option<<super::UvIndexType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UvHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                index: None,
                state: Box::new(UvHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: UvHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use UvHourlyTypeDeserializerState as S;
            match state {
                S::Index(Some(deserializer)) => self.store_index(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_index(&mut self, value: super::UvIndexType) -> Result<(), Error> {
            if self.index.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"index",
                )))?;
            }
            self.index = Some(value);
            Ok(())
        }
        fn handle_index<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::UvIndexType>,
            fallback: &mut Option<UvHourlyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.index.is_some() {
                    fallback.get_or_insert(UvHourlyTypeDeserializerState::Index(None));
                    *self.state = UvHourlyTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = UvHourlyTypeDeserializerState::Index(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_index(data)?;
                    *self.state = UvHourlyTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(UvHourlyTypeDeserializerState::Index(Some(
                                deserializer,
                            )));
                            *self.state = UvHourlyTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = UvHourlyTypeDeserializerState::Index(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UvHourlyType> for UvHourlyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::UvHourlyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UvHourlyType>
        where
            R: DeserializeReader,
        {
            use UvHourlyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Index(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_index(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = UvHourlyTypeDeserializerState::Index(None);
                        event
                    }
                    (S::Index(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"index", false)?;
                        match self.handle_index(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::UvHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, UvHourlyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::UvHourlyType {
                index: self
                    .index
                    .ok_or_else(|| ErrorKind::MissingElement("index".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct UvTypeDeserializer {
        category: Option<super::UvCategoryType>,
        index: Option<super::UvIndexType>,
        text_summary: Option<::std::string::String>,
        state: Box<UvTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UvTypeDeserializerState {
        Init__,
        Index(Option<<super::UvIndexType as WithDeserializer>::Deserializer>),
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UvTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut category: Option<super::UvCategoryType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"category" {
                    reader.read_attrib(&mut category, b"category", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                category: category,
                index: None,
                text_summary: None,
                state: Box::new(UvTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: UvTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use UvTypeDeserializerState as S;
            match state {
                S::Index(Some(deserializer)) => self.store_index(deserializer.finish(reader)?)?,
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_index(&mut self, value: super::UvIndexType) -> Result<(), Error> {
            if self.index.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"index",
                )))?;
            }
            self.index = Some(value);
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_index<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::UvIndexType>,
            fallback: &mut Option<UvTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.index.is_some() {
                    fallback.get_or_insert(UvTypeDeserializerState::Index(None));
                    *self.state = UvTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = UvTypeDeserializerState::Index(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_index(data)?;
                    *self.state = UvTypeDeserializerState::TextSummary(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(UvTypeDeserializerState::Index(Some(deserializer)));
                            *self.state = UvTypeDeserializerState::TextSummary(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = UvTypeDeserializerState::Index(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<UvTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(UvTypeDeserializerState::TextSummary(None));
                    *self.state = UvTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = UvTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = UvTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(UvTypeDeserializerState::TextSummary(Some(
                                deserializer,
                            )));
                            *self.state = UvTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = UvTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UvType> for UvTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::UvType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UvType>
        where
            R: DeserializeReader,
        {
            use UvTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Index(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_index(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = UvTypeDeserializerState::Index(None);
                        event
                    }
                    (S::Index(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"index", false)?;
                        match self.handle_index(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::UvType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, UvTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::UvType {
                category: self.category,
                index: self
                    .index
                    .ok_or_else(|| ErrorKind::MissingElement("index".into()))?,
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ValidWindHourlyDirectionsTypeDeserializer {
        units: Option<super::UnitsHourlyType>,
        wind_dir_full: Option<super::WindDirFullType>,
        content: Option<super::ValidWindDirectionsType>,
        state: Box<ValidWindHourlyDirectionsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ValidWindHourlyDirectionsTypeDeserializerState {
        Init__,
        Content__(<super::ValidWindDirectionsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ValidWindHourlyDirectionsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::UnitsHourlyType> = None;
            let mut wind_dir_full: Option<super::WindDirFullType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"windDirFull" {
                    reader.read_attrib(&mut wind_dir_full, b"windDirFull", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                wind_dir_full: wind_dir_full,
                content: None,
                state: Box::new(ValidWindHourlyDirectionsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ValidWindHourlyDirectionsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ValidWindHourlyDirectionsTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidWindDirectionsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindDirectionsType>,
        ) -> DeserializerResult<'de, super::ValidWindHourlyDirectionsType>
        where
            R: DeserializeReader,
        {
            use ValidWindHourlyDirectionsTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ValidWindHourlyDirectionsType>
        for ValidWindHourlyDirectionsTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ValidWindHourlyDirectionsType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ValidWindHourlyDirectionsType>
        where
            R: DeserializeReader,
        {
            use ValidWindHourlyDirectionsTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ValidWindHourlyDirectionsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ValidWindHourlyDirectionsTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ValidWindHourlyDirectionsType {
                units: self.units,
                wind_dir_full: self.wind_dir_full,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct VisibilitySubTypeForecastTypeDeserializer {
        cause: Option<::std::string::String>,
        text_summary: Option<::std::string::String>,
        state: Box<VisibilitySubTypeForecastTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum VisibilitySubTypeForecastTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl VisibilitySubTypeForecastTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut cause: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"cause" {
                    reader.read_attrib(&mut cause, b"cause", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                cause: cause,
                text_summary: None,
                state: Box::new(VisibilitySubTypeForecastTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: VisibilitySubTypeForecastTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use VisibilitySubTypeForecastTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<VisibilitySubTypeForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.text_summary.is_some() {
                    fallback.get_or_insert(
                        VisibilitySubTypeForecastTypeDeserializerState::TextSummary(None),
                    );
                    *self.state = VisibilitySubTypeForecastTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = VisibilitySubTypeForecastTypeDeserializerState::TextSummary(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = VisibilitySubTypeForecastTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                VisibilitySubTypeForecastTypeDeserializerState::TextSummary(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = VisibilitySubTypeForecastTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                VisibilitySubTypeForecastTypeDeserializerState::TextSummary(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::VisibilitySubTypeForecastType>
        for VisibilitySubTypeForecastTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VisibilitySubTypeForecastType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VisibilitySubTypeForecastType>
        where
            R: DeserializeReader,
        {
            use VisibilitySubTypeForecastTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            VisibilitySubTypeForecastTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::VisibilitySubTypeForecastType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                VisibilitySubTypeForecastTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::VisibilitySubTypeForecastType {
                cause: self.cause,
                text_summary: self
                    .text_summary
                    .ok_or_else(|| ErrorKind::MissingElement("textSummary".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct VisibilityTypeCondTypeDeserializer {
        units: Option<super::ValidVisibilityUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidVisibilitiesType>,
        state: Box<VisibilityTypeCondTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum VisibilityTypeCondTypeDeserializerState {
        Init__,
        Content__(<super::ValidVisibilitiesType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl VisibilityTypeCondTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidVisibilityUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                qa_value: qa_value,
                content: None,
                state: Box::new(VisibilityTypeCondTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: VisibilityTypeCondTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let VisibilityTypeCondTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidVisibilitiesType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidVisibilitiesType>,
        ) -> DeserializerResult<'de, super::VisibilityTypeCondType>
        where
            R: DeserializeReader,
        {
            use VisibilityTypeCondTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::VisibilityTypeCondType> for VisibilityTypeCondTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VisibilityTypeCondType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VisibilityTypeCondType>
        where
            R: DeserializeReader,
        {
            use VisibilityTypeCondTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::VisibilityTypeCondType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                VisibilityTypeCondTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::VisibilityTypeCondType {
                units: self.units,
                unit_type: self.unit_type,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct VisibilityTypeForecastTypeDeserializer {
        wind_visib: Option<super::VisibilitySubTypeForecastType>,
        other_visib: Option<super::VisibilitySubTypeForecastType>,
        state: Box<VisibilityTypeForecastTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum VisibilityTypeForecastTypeDeserializerState {
        Init__,
        WindVisib(Option<<super::VisibilitySubTypeForecastType as WithDeserializer>::Deserializer>),
        OtherVisib(
            Option<<super::VisibilitySubTypeForecastType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl VisibilityTypeForecastTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                wind_visib: None,
                other_visib: None,
                state: Box::new(VisibilityTypeForecastTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: VisibilityTypeForecastTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use VisibilityTypeForecastTypeDeserializerState as S;
            match state {
                S::WindVisib(Some(deserializer)) => {
                    self.store_wind_visib(deserializer.finish(reader)?)?
                }
                S::OtherVisib(Some(deserializer)) => {
                    self.store_other_visib(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_wind_visib(
            &mut self,
            value: super::VisibilitySubTypeForecastType,
        ) -> Result<(), Error> {
            if self.wind_visib.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"windVisib",
                )))?;
            }
            self.wind_visib = Some(value);
            Ok(())
        }
        fn store_other_visib(
            &mut self,
            value: super::VisibilitySubTypeForecastType,
        ) -> Result<(), Error> {
            if self.other_visib.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"otherVisib",
                )))?;
            }
            self.other_visib = Some(value);
            Ok(())
        }
        fn handle_wind_visib<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::VisibilitySubTypeForecastType>,
            fallback: &mut Option<VisibilityTypeForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback
                    .get_or_insert(VisibilityTypeForecastTypeDeserializerState::WindVisib(None));
                *self.state = VisibilityTypeForecastTypeDeserializerState::OtherVisib(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind_visib(data)?;
                    *self.state = VisibilityTypeForecastTypeDeserializerState::OtherVisib(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                VisibilityTypeForecastTypeDeserializerState::WindVisib(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                VisibilityTypeForecastTypeDeserializerState::OtherVisib(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = VisibilityTypeForecastTypeDeserializerState::WindVisib(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_other_visib<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::VisibilitySubTypeForecastType>,
            fallback: &mut Option<VisibilityTypeForecastTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(VisibilityTypeForecastTypeDeserializerState::OtherVisib(
                    None,
                ));
                *self.state = VisibilityTypeForecastTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_other_visib(data)?;
                    *self.state = VisibilityTypeForecastTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                VisibilityTypeForecastTypeDeserializerState::OtherVisib(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = VisibilityTypeForecastTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = VisibilityTypeForecastTypeDeserializerState::OtherVisib(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::VisibilityTypeForecastType>
        for VisibilityTypeForecastTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VisibilityTypeForecastType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VisibilityTypeForecastType>
        where
            R: DeserializeReader,
        {
            use VisibilityTypeForecastTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::WindVisib(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind_visib(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::OtherVisib(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_other_visib(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = VisibilityTypeForecastTypeDeserializerState::WindVisib(None);
                        event
                    }
                    (S::WindVisib(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"windVisib", false)?;
                        match self.handle_wind_visib(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::OtherVisib(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"otherVisib",
                            false,
                        )?;
                        match self.handle_other_visib(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::VisibilityTypeForecastType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                VisibilityTypeForecastTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::VisibilityTypeForecastType {
                wind_visib: self.wind_visib,
                other_visib: self.other_visib,
            })
        }
    }
    #[derive(Debug)]
    pub struct WarningEventTypeDeserializer {
        type_: Option<super::ValidWarningTypesType>,
        description: Option<::std::string::String>,
        priority: Option<super::ValidWarningPrioritiesType>,
        expiry_time: Option<TimeStampType>,
        url: Option<::std::string::String>,
        date_time: Vec<super::DateStampType>,
        state: Box<WarningEventTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WarningEventTypeDeserializerState {
        Init__,
        DateTime(Option<<super::DateStampType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WarningEventTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut type_: Option<super::ValidWarningTypesType> = None;
            let mut description: Option<::std::string::String> = None;
            let mut priority: Option<super::ValidWarningPrioritiesType> = None;
            let mut expiry_time: Option<TimeStampType> = None;
            let mut url: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"description" {
                    reader.read_attrib(&mut description, b"description", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"priority" {
                    reader.read_attrib(&mut priority, b"priority", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"expiryTime" {
                    reader.read_attrib(&mut expiry_time, b"expiryTime", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"url" {
                    reader.read_attrib(&mut url, b"url", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                type_: type_,
                description: description,
                priority: priority,
                expiry_time: expiry_time,
                url: url,
                date_time: Vec::new(),
                state: Box::new(WarningEventTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WarningEventTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WarningEventTypeDeserializerState as S;
            match state {
                S::DateTime(Some(deserializer)) => {
                    self.store_date_time(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_date_time(&mut self, value: super::DateStampType) -> Result<(), Error> {
            self.date_time.push(value);
            Ok(())
        }
        fn handle_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateStampType>,
            fallback: &mut Option<WarningEventTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.date_time.len() < 2usize {
                    *self.state = WarningEventTypeDeserializerState::DateTime(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(WarningEventTypeDeserializerState::DateTime(None));
                    *self.state = WarningEventTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time(data)?;
                    if self.date_time.len() < 2usize {
                        *self.state = WarningEventTypeDeserializerState::DateTime(None);
                    } else {
                        *self.state = WarningEventTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WarningEventTypeDeserializerState::DateTime(
                                Some(deserializer),
                            ));
                            if self.date_time.len().saturating_add(1) < 2usize {
                                *self.state = WarningEventTypeDeserializerState::DateTime(None);
                            } else {
                                *self.state = WarningEventTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WarningEventTypeDeserializerState::DateTime(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WarningEventType> for WarningEventTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WarningEventType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WarningEventType>
        where
            R: DeserializeReader,
        {
            use WarningEventTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WarningEventTypeDeserializerState::DateTime(None);
                        event
                    }
                    (S::DateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"dateTime", false)?;
                        match self.handle_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WarningEventType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                WarningEventTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::WarningEventType {
                type_: self.type_,
                description: self.description,
                priority: self.priority,
                expiry_time: self.expiry_time,
                url: self.url,
                date_time: self.date_time.try_into().map_err(|vec: Vec<_>| {
                    ErrorKind::InsufficientSize {
                        min: 2usize,
                        max: 2usize,
                        actual: vec.len(),
                    }
                })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WarningsTypeDeserializer {
        url: Option<::std::string::String>,
        event: Option<super::WarningEventType>,
        state: Box<WarningsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WarningsTypeDeserializerState {
        Init__,
        Event(Option<<super::WarningEventType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WarningsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut url: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"url" {
                    reader.read_attrib(&mut url, b"url", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                url: url,
                event: None,
                state: Box::new(WarningsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WarningsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WarningsTypeDeserializerState as S;
            match state {
                S::Event(Some(deserializer)) => self.store_event(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_event(&mut self, value: super::WarningEventType) -> Result<(), Error> {
            if self.event.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"event",
                )))?;
            }
            self.event = Some(value);
            Ok(())
        }
        fn handle_event<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WarningEventType>,
            fallback: &mut Option<WarningsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WarningsTypeDeserializerState::Event(None));
                *self.state = WarningsTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_event(data)?;
                    *self.state = WarningsTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WarningsTypeDeserializerState::Event(Some(
                                deserializer,
                            )));
                            *self.state = WarningsTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WarningsTypeDeserializerState::Event(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WarningsType> for WarningsTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WarningsType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WarningsType>
        where
            R: DeserializeReader,
        {
            use WarningsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Event(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_event(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WarningsTypeDeserializerState::Event(None);
                        event
                    }
                    (S::Event(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"event", false)?;
                        match self.handle_event(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WarningsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WarningsTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::WarningsType {
                url: self.url,
                event: self.event,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindBearingTypeDeserializer {
        units: Option<super::ValidWindBearingUnitsType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidWindBearingsType>,
        state: Box<WindBearingTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindBearingTypeDeserializerState {
        Init__,
        Content__(<super::ValidWindBearingsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl WindBearingTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidWindBearingUnitsType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                qa_value: qa_value,
                content: None,
                state: Box::new(WindBearingTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindBearingTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let WindBearingTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidWindBearingsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindBearingsType>,
        ) -> DeserializerResult<'de, super::WindBearingType>
        where
            R: DeserializeReader,
        {
            use WindBearingTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WindBearingType> for WindBearingTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WindBearingType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindBearingType>
        where
            R: DeserializeReader,
        {
            use WindBearingTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindBearingType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                WindBearingTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::WindBearingType {
                units: self.units,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindChillHourlyTypeDeserializer {
        unit_type: Option<super::ValidUnitTypesType>,
        content: Option<super::ValidWindChillsType>,
        state: Box<WindChillHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindChillHourlyTypeDeserializerState {
        Init__,
        Content__(<super::ValidWindChillsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl WindChillHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                unit_type: unit_type,
                content: None,
                state: Box::new(WindChillHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindChillHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let WindChillHourlyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidWindChillsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindChillsType>,
        ) -> DeserializerResult<'de, super::WindChillHourlyType>
        where
            R: DeserializeReader,
        {
            use WindChillHourlyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WindChillHourlyType> for WindChillHourlyTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindChillHourlyType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindChillHourlyType>
        where
            R: DeserializeReader,
        {
            use WindChillHourlyTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindChillHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                WindChillHourlyTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::WindChillHourlyType {
                unit_type: self.unit_type,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindChillTypeDeserializer {
        text_summary: Option<::std::string::String>,
        calculated: Vec<super::CalculatedWindChillType>,
        frostbite: Option<super::FrostbiteWindChillType>,
        state: Box<WindChillTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindChillTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Calculated(Option<<super::CalculatedWindChillType as WithDeserializer>::Deserializer>),
        Frostbite(Option<<super::FrostbiteWindChillType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WindChillTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                calculated: Vec::new(),
                frostbite: None,
                state: Box::new(WindChillTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindChillTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WindChillTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::Calculated(Some(deserializer)) => {
                    self.store_calculated(deserializer.finish(reader)?)?
                }
                S::Frostbite(Some(deserializer)) => {
                    self.store_frostbite(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_calculated(&mut self, value: super::CalculatedWindChillType) -> Result<(), Error> {
            self.calculated.push(value);
            Ok(())
        }
        fn store_frostbite(&mut self, value: super::FrostbiteWindChillType) -> Result<(), Error> {
            if self.frostbite.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"frostbite",
                )))?;
            }
            self.frostbite = Some(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<WindChillTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WindChillTypeDeserializerState::TextSummary(None));
                *self.state = WindChillTypeDeserializerState::Calculated(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = WindChillTypeDeserializerState::Calculated(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindChillTypeDeserializerState::TextSummary(
                                Some(deserializer),
                            ));
                            *self.state = WindChillTypeDeserializerState::Calculated(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WindChillTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_calculated<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CalculatedWindChillType>,
            fallback: &mut Option<WindChillTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WindChillTypeDeserializerState::Calculated(None));
                *self.state = WindChillTypeDeserializerState::Frostbite(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_calculated(data)?;
                    if self.calculated.len() < 2usize {
                        *self.state = WindChillTypeDeserializerState::Calculated(None);
                    } else {
                        *self.state = WindChillTypeDeserializerState::Frostbite(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindChillTypeDeserializerState::Calculated(
                                Some(deserializer),
                            ));
                            *self.state = WindChillTypeDeserializerState::Calculated(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WindChillTypeDeserializerState::Calculated(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_frostbite<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FrostbiteWindChillType>,
            fallback: &mut Option<WindChillTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WindChillTypeDeserializerState::Frostbite(None));
                *self.state = WindChillTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_frostbite(data)?;
                    *self.state = WindChillTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindChillTypeDeserializerState::Frostbite(
                                Some(deserializer),
                            ));
                            *self.state = WindChillTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WindChillTypeDeserializerState::Frostbite(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WindChillType> for WindChillTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WindChillType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindChillType>
        where
            R: DeserializeReader,
        {
            use WindChillTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Calculated(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_calculated(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Frostbite(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_frostbite(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WindChillTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Calculated(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"calculated",
                            false,
                        )?;
                        match self.handle_calculated(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Frostbite(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"frostbite", false)?;
                        match self.handle_frostbite(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindChillType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WindChillTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::WindChillType {
                text_summary: self.text_summary,
                calculated: self.calculated,
                frostbite: self.frostbite,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindDirectionTypeDeserializer {
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidWindDirectionsType>,
        state: Box<WindDirectionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindDirectionTypeDeserializerState {
        Init__,
        Content__(<super::ValidWindDirectionsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl WindDirectionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                qa_value: qa_value,
                content: None,
                state: Box::new(WindDirectionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindDirectionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let WindDirectionTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidWindDirectionsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindDirectionsType>,
        ) -> DeserializerResult<'de, super::WindDirectionType>
        where
            R: DeserializeReader,
        {
            use WindDirectionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WindDirectionType> for WindDirectionTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindDirectionType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindDirectionType>
        where
            R: DeserializeReader,
        {
            use WindDirectionTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindDirectionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                WindDirectionTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::WindDirectionType {
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindHourlyTypeDeserializer {
        speed: Option<super::WindSpeedType>,
        direction: Option<super::ValidWindHourlyDirectionsType>,
        gust: Option<super::WindSpeedType>,
        state: Box<WindHourlyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindHourlyTypeDeserializerState {
        Init__,
        Speed(Option<<super::WindSpeedType as WithDeserializer>::Deserializer>),
        Direction(Option<<super::ValidWindHourlyDirectionsType as WithDeserializer>::Deserializer>),
        Gust(Option<<super::WindSpeedType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WindHourlyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                speed: None,
                direction: None,
                gust: None,
                state: Box::new(WindHourlyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindHourlyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WindHourlyTypeDeserializerState as S;
            match state {
                S::Speed(Some(deserializer)) => self.store_speed(deserializer.finish(reader)?)?,
                S::Direction(Some(deserializer)) => {
                    self.store_direction(deserializer.finish(reader)?)?
                }
                S::Gust(Some(deserializer)) => self.store_gust(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_speed(&mut self, value: super::WindSpeedType) -> Result<(), Error> {
            if self.speed.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"speed",
                )))?;
            }
            self.speed = Some(value);
            Ok(())
        }
        fn store_direction(
            &mut self,
            value: super::ValidWindHourlyDirectionsType,
        ) -> Result<(), Error> {
            if self.direction.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"direction",
                )))?;
            }
            self.direction = Some(value);
            Ok(())
        }
        fn store_gust(&mut self, value: super::WindSpeedType) -> Result<(), Error> {
            if self.gust.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"gust")))?;
            }
            self.gust = Some(value);
            Ok(())
        }
        fn handle_speed<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindSpeedType>,
            fallback: &mut Option<WindHourlyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.speed.is_some() {
                    fallback.get_or_insert(WindHourlyTypeDeserializerState::Speed(None));
                    *self.state = WindHourlyTypeDeserializerState::Direction(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindHourlyTypeDeserializerState::Speed(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_speed(data)?;
                    *self.state = WindHourlyTypeDeserializerState::Direction(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindHourlyTypeDeserializerState::Speed(Some(
                                deserializer,
                            )));
                            *self.state = WindHourlyTypeDeserializerState::Direction(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WindHourlyTypeDeserializerState::Speed(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_direction<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindHourlyDirectionsType>,
            fallback: &mut Option<WindHourlyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.direction.is_some() {
                    fallback.get_or_insert(WindHourlyTypeDeserializerState::Direction(None));
                    *self.state = WindHourlyTypeDeserializerState::Gust(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindHourlyTypeDeserializerState::Direction(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_direction(data)?;
                    *self.state = WindHourlyTypeDeserializerState::Gust(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindHourlyTypeDeserializerState::Direction(
                                Some(deserializer),
                            ));
                            *self.state = WindHourlyTypeDeserializerState::Gust(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WindHourlyTypeDeserializerState::Direction(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_gust<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindSpeedType>,
            fallback: &mut Option<WindHourlyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.gust.is_some() {
                    fallback.get_or_insert(WindHourlyTypeDeserializerState::Gust(None));
                    *self.state = WindHourlyTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindHourlyTypeDeserializerState::Gust(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_gust(data)?;
                    *self.state = WindHourlyTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindHourlyTypeDeserializerState::Gust(Some(
                                deserializer,
                            )));
                            *self.state = WindHourlyTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WindHourlyTypeDeserializerState::Gust(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WindHourlyType> for WindHourlyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WindHourlyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindHourlyType>
        where
            R: DeserializeReader,
        {
            use WindHourlyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Speed(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_speed(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Direction(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_direction(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Gust(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_gust(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WindHourlyTypeDeserializerState::Speed(None);
                        event
                    }
                    (S::Speed(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"speed", false)?;
                        match self.handle_speed(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Direction(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"direction", false)?;
                        match self.handle_direction(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Gust(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"gust", false)?;
                        match self.handle_gust(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindHourlyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WindHourlyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::WindHourlyType {
                speed: self
                    .speed
                    .ok_or_else(|| ErrorKind::MissingElement("speed".into()))?,
                direction: self
                    .direction
                    .ok_or_else(|| ErrorKind::MissingElement("direction".into()))?,
                gust: self
                    .gust
                    .ok_or_else(|| ErrorKind::MissingElement("gust".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindSpeedTypeDeserializer {
        units: Option<super::ValidWindUnitsType>,
        unit_type: Option<super::ValidUnitTypesType>,
        qa_value: Option<super::QaValueType>,
        content: Option<super::ValidWindSpeedsType>,
        state: Box<WindSpeedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindSpeedTypeDeserializerState {
        Init__,
        Content__(<super::ValidWindSpeedsType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl WindSpeedTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut units: Option<super::ValidWindUnitsType> = None;
            let mut unit_type: Option<super::ValidUnitTypesType> = None;
            let mut qa_value: Option<super::QaValueType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"units" {
                    reader.read_attrib(&mut units, b"units", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"unitType" {
                    reader.read_attrib(&mut unit_type, b"unitType", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"qaValue" {
                    reader.read_attrib(&mut qa_value, b"qaValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                units: units,
                unit_type: unit_type,
                qa_value: qa_value,
                content: None,
                state: Box::new(WindSpeedTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindSpeedTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let WindSpeedTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ValidWindSpeedsType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ValidWindSpeedsType>,
        ) -> DeserializerResult<'de, super::WindSpeedType>
        where
            R: DeserializeReader,
        {
            use WindSpeedTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WindSpeedType> for WindSpeedTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WindSpeedType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindSpeedType>
        where
            R: DeserializeReader,
        {
            use WindSpeedTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindSpeedType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WindSpeedTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::WindSpeedType {
                units: self.units,
                unit_type: self.unit_type,
                qa_value: self.qa_value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindTypeDeserializer {
        index: Option<super::ValidWindIndicesType>,
        rank: Option<super::ValidWindRanksType>,
        speed: Option<super::WindSpeedType>,
        gust: Option<super::WindSpeedType>,
        direction: Option<super::WindDirectionType>,
        bearing: Option<super::WindBearingType>,
        state: Box<WindTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindTypeDeserializerState {
        Init__,
        Speed(Option<<super::WindSpeedType as WithDeserializer>::Deserializer>),
        Gust(Option<<super::WindSpeedType as WithDeserializer>::Deserializer>),
        Direction(Option<<super::WindDirectionType as WithDeserializer>::Deserializer>),
        Bearing(Option<<super::WindBearingType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WindTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut index: Option<super::ValidWindIndicesType> = None;
            let mut rank: Option<super::ValidWindRanksType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"index" {
                    reader.read_attrib(&mut index, b"index", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"rank" {
                    reader.read_attrib(&mut rank, b"rank", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                index: index,
                rank: rank,
                speed: None,
                gust: None,
                direction: None,
                bearing: None,
                state: Box::new(WindTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WindTypeDeserializerState as S;
            match state {
                S::Speed(Some(deserializer)) => self.store_speed(deserializer.finish(reader)?)?,
                S::Gust(Some(deserializer)) => self.store_gust(deserializer.finish(reader)?)?,
                S::Direction(Some(deserializer)) => {
                    self.store_direction(deserializer.finish(reader)?)?
                }
                S::Bearing(Some(deserializer)) => {
                    self.store_bearing(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_speed(&mut self, value: super::WindSpeedType) -> Result<(), Error> {
            if self.speed.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"speed",
                )))?;
            }
            self.speed = Some(value);
            Ok(())
        }
        fn store_gust(&mut self, value: super::WindSpeedType) -> Result<(), Error> {
            if self.gust.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"gust")))?;
            }
            self.gust = Some(value);
            Ok(())
        }
        fn store_direction(&mut self, value: super::WindDirectionType) -> Result<(), Error> {
            if self.direction.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"direction",
                )))?;
            }
            self.direction = Some(value);
            Ok(())
        }
        fn store_bearing(&mut self, value: super::WindBearingType) -> Result<(), Error> {
            if self.bearing.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"bearing",
                )))?;
            }
            self.bearing = Some(value);
            Ok(())
        }
        fn handle_speed<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindSpeedType>,
            fallback: &mut Option<WindTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.speed.is_some() {
                    fallback.get_or_insert(WindTypeDeserializerState::Speed(None));
                    *self.state = WindTypeDeserializerState::Gust(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindTypeDeserializerState::Speed(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_speed(data)?;
                    *self.state = WindTypeDeserializerState::Gust(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindTypeDeserializerState::Speed(Some(
                                deserializer,
                            )));
                            *self.state = WindTypeDeserializerState::Gust(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WindTypeDeserializerState::Speed(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_gust<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindSpeedType>,
            fallback: &mut Option<WindTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.gust.is_some() {
                    fallback.get_or_insert(WindTypeDeserializerState::Gust(None));
                    *self.state = WindTypeDeserializerState::Direction(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindTypeDeserializerState::Gust(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_gust(data)?;
                    *self.state = WindTypeDeserializerState::Direction(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(WindTypeDeserializerState::Gust(Some(deserializer)));
                            *self.state = WindTypeDeserializerState::Direction(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WindTypeDeserializerState::Gust(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_direction<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindDirectionType>,
            fallback: &mut Option<WindTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.direction.is_some() {
                    fallback.get_or_insert(WindTypeDeserializerState::Direction(None));
                    *self.state = WindTypeDeserializerState::Bearing(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindTypeDeserializerState::Direction(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_direction(data)?;
                    *self.state = WindTypeDeserializerState::Bearing(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindTypeDeserializerState::Direction(Some(
                                deserializer,
                            )));
                            *self.state = WindTypeDeserializerState::Bearing(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WindTypeDeserializerState::Direction(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bearing<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindBearingType>,
            fallback: &mut Option<WindTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.bearing.is_some() {
                    fallback.get_or_insert(WindTypeDeserializerState::Bearing(None));
                    *self.state = WindTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = WindTypeDeserializerState::Bearing(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bearing(data)?;
                    *self.state = WindTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindTypeDeserializerState::Bearing(Some(
                                deserializer,
                            )));
                            *self.state = WindTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WindTypeDeserializerState::Bearing(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WindType> for WindTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WindType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindType>
        where
            R: DeserializeReader,
        {
            use WindTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Speed(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_speed(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Gust(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_gust(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Direction(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_direction(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bearing(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bearing(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WindTypeDeserializerState::Speed(None);
                        event
                    }
                    (S::Speed(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"speed", false)?;
                        match self.handle_speed(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Gust(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"gust", false)?;
                        match self.handle_gust(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Direction(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"direction", false)?;
                        match self.handle_direction(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bearing(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"bearing", false)?;
                        match self.handle_bearing(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WindTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::WindType {
                index: self.index,
                rank: self.rank,
                speed: self
                    .speed
                    .ok_or_else(|| ErrorKind::MissingElement("speed".into()))?,
                gust: self
                    .gust
                    .ok_or_else(|| ErrorKind::MissingElement("gust".into()))?,
                direction: self
                    .direction
                    .ok_or_else(|| ErrorKind::MissingElement("direction".into()))?,
                bearing: self
                    .bearing
                    .ok_or_else(|| ErrorKind::MissingElement("bearing".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WindsTypeDeserializer {
        text_summary: Option<::std::string::String>,
        wind: Vec<super::WindType>,
        state: Box<WindsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WindsTypeDeserializerState {
        Init__,
        TextSummary(Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Wind(Option<<super::WindType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WindsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_summary: None,
                wind: Vec::new(),
                state: Box::new(WindsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WindsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WindsTypeDeserializerState as S;
            match state {
                S::TextSummary(Some(deserializer)) => {
                    self.store_text_summary(deserializer.finish(reader)?)?
                }
                S::Wind(Some(deserializer)) => self.store_wind(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_text_summary(&mut self, value: ::std::string::String) -> Result<(), Error> {
            if self.text_summary.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"textSummary",
                )))?;
            }
            self.text_summary = Some(value);
            Ok(())
        }
        fn store_wind(&mut self, value: super::WindType) -> Result<(), Error> {
            self.wind.push(value);
            Ok(())
        }
        fn handle_text_summary<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut Option<WindsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WindsTypeDeserializerState::TextSummary(None));
                *self.state = WindsTypeDeserializerState::Wind(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_summary(data)?;
                    *self.state = WindsTypeDeserializerState::Wind(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindsTypeDeserializerState::TextSummary(Some(
                                deserializer,
                            )));
                            *self.state = WindsTypeDeserializerState::Wind(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WindsTypeDeserializerState::TextSummary(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_wind<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WindType>,
            fallback: &mut Option<WindsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WindsTypeDeserializerState::Wind(None));
                *self.state = WindsTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_wind(data)?;
                    *self.state = WindsTypeDeserializerState::Wind(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WindsTypeDeserializerState::Wind(Some(
                                deserializer,
                            )));
                            *self.state = WindsTypeDeserializerState::Wind(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = WindsTypeDeserializerState::Wind(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WindsType> for WindsTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WindsType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WindsType>
        where
            R: DeserializeReader,
        {
            use WindsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextSummary(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Wind(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_wind(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WindsTypeDeserializerState::TextSummary(None);
                        event
                    }
                    (S::TextSummary(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"textSummary",
                            false,
                        )?;
                        match self.handle_text_summary(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Wind(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"wind", false)?;
                        match self.handle_wind(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WindsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WindsTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::WindsType {
                text_summary: self.text_summary,
                wind: self.wind,
            })
        }
    }
}
pub mod xs {
    use xsd_parser::quick_xml::{DeserializeBytes, DeserializeReader, Error, WithDeserializer};
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<::std::string::String>);
    impl DeserializeBytes for EntitiesType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| ::std::string::String::deserialize_bytes(reader, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    #[derive(Debug, Default)]
    pub struct EntityType(pub Vec<::std::string::String>);
    impl DeserializeBytes for EntityType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| ::std::string::String::deserialize_bytes(reader, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type IdType = ::std::string::String;
    pub type IdrefType = ::std::string::String;
    #[derive(Debug, Default)]
    pub struct IdrefsType(pub Vec<::std::string::String>);
    impl DeserializeBytes for IdrefsType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| ::std::string::String::deserialize_bytes(reader, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type NcNameType = ::std::string::String;
    pub type NmtokenType = ::std::string::String;
    #[derive(Debug, Default)]
    pub struct NmtokensType(pub Vec<::std::string::String>);
    impl DeserializeBytes for NmtokensType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| ::std::string::String::deserialize_bytes(reader, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type NotationType = ::std::string::String;
    pub type NameType = ::std::string::String;
    pub type QNameType = ::std::string::String;
    pub type AnySimpleType = ::std::string::String;
    #[derive(Debug)]
    pub struct AnyType;
    impl WithDeserializer for AnyType {
        type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
    }
    pub type AnyUriType = ::std::string::String;
    pub type Base64BinaryType = ::std::string::String;
    pub type BooleanType = ::core::primitive::bool;
    pub type ByteType = ::core::primitive::i8;
    pub type DateType = ::std::string::String;
    pub type DateTimeType = ::std::string::String;
    pub type DecimalType = ::core::primitive::f64;
    pub type DoubleType = ::core::primitive::f64;
    pub type DurationType = ::std::string::String;
    pub type FloatType = ::core::primitive::f32;
    pub type GDayType = ::std::string::String;
    pub type GMonthType = ::std::string::String;
    pub type GMonthDayType = ::std::string::String;
    pub type GYearType = ::std::string::String;
    pub type GYearMonthType = ::std::string::String;
    pub type HexBinaryType = ::std::string::String;
    pub type IntType = ::core::primitive::i32;
    pub type IntegerType = ::core::primitive::i32;
    pub type LanguageType = ::std::string::String;
    pub type LongType = ::core::primitive::i64;
    pub type NegativeIntegerType = ::core::primitive::isize;
    pub type NonNegativeIntegerType = ::core::primitive::usize;
    pub type NonPositiveIntegerType = ::core::primitive::isize;
    pub type NormalizedStringType = ::std::string::String;
    pub type PositiveIntegerType = ::core::primitive::usize;
    pub type ShortType = ::core::primitive::i16;
    pub type StringType = ::std::string::String;
    pub type TimeType = ::std::string::String;
    pub type TokenType = ::std::string::String;
    pub type UnsignedByteType = ::core::primitive::u8;
    pub type UnsignedIntType = ::core::primitive::u32;
    pub type UnsignedLongType = ::core::primitive::u64;
    pub type UnsignedShortType = ::core::primitive::u16;
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser::quick_xml::{
            BytesStart, DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, Error, Event,
        };
        #[derive(Debug)]
        pub struct AnyTypeDeserializer {
            state: Box<AnyTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum AnyTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl AnyTypeDeserializer {
            fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
            where
                R: DeserializeReader,
            {
                Ok(Self {
                    state: Box::new(AnyTypeDeserializerState::Init__),
                })
            }
            fn finish_state<R>(
                &mut self,
                reader: &R,
                state: AnyTypeDeserializerState,
            ) -> Result<(), Error>
            where
                R: DeserializeReader,
            {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
            fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnyType>
            where
                R: DeserializeReader,
            {
                reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnyType>
            where
                R: DeserializeReader,
            {
                if let Event::End(_) = &event {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    })
                } else {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::Break(event),
                        allow_any: true,
                    })
                }
            }
            fn finish<R>(mut self, reader: &R) -> Result<super::AnyType, Error>
            where
                R: DeserializeReader,
            {
                let state = replace(&mut *self.state, AnyTypeDeserializerState::Unknown__);
                self.finish_state(reader, state)?;
                Ok(super::AnyType {})
            }
        }
    }
}
