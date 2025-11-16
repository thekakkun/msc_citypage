use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurrentConditionIcon {
    Sunny = 0,
    MainlySunny = 1,
    PartlyCloudy = 2,
    MostlyCloudy = 3,
    LightRainShower = 6,
    LightRainShowerAndFlurries = 7,
    LightFlurries = 8,
    Cloudy = 10,
    Precipitation = 11,
    RainShower = 12,
    Rain = 13,
    FreezingRain = 14,
    RainAndFlurries = 15,
    LightSnow = 16,
    Flurries = 17,
    HeavySnow = 18,
    Thunderstorm = 19,
    Haze = 23,
    Fog = 24,
    DriftingSnow = 25,
    IceCrystals = 26,
    IcePellets = 27,
    Drizzle = 28,
    Clear = 30,
    MainlyClear = 31,
    PartlyCloudyNight = 32,
    MostlyCloudyNight = 33,
    LightRainShowerNight = 36,
    LightRainShowerAndFlurriesNight = 37,
    LightFlurriesNight = 38,
    ThunderstormNight = 39,
    BlowingSnow = 40,
    FunnelCloud = 41,
    Tornado = 42,
    Windy = 43,
    Smoke = 44,
    DustStorm = 45,
    ThunderstormWithHail = 46,
    ThunderstormWithDust = 47,
    Waterspout = 48,
}

impl CurrentConditionIcon {
    pub fn code(&self) -> u8 {
        *self as u8
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Sunny),
            1 => Some(Self::MainlySunny),
            2 => Some(Self::PartlyCloudy),
            3 => Some(Self::MostlyCloudy),
            6 => Some(Self::LightRainShower),
            7 => Some(Self::LightRainShowerAndFlurries),
            8 => Some(Self::LightFlurries),
            10 => Some(Self::Cloudy),
            11 => Some(Self::Precipitation),
            12 => Some(Self::RainShower),
            13 => Some(Self::Rain),
            14 => Some(Self::FreezingRain),
            15 => Some(Self::RainAndFlurries),
            16 => Some(Self::LightSnow),
            17 => Some(Self::Flurries),
            18 => Some(Self::HeavySnow),
            19 => Some(Self::Thunderstorm),
            23 => Some(Self::Haze),
            24 => Some(Self::Fog),
            25 => Some(Self::DriftingSnow),
            26 => Some(Self::IceCrystals),
            27 => Some(Self::IcePellets),
            28 => Some(Self::Drizzle),
            30 => Some(Self::Clear),
            31 => Some(Self::MainlyClear),
            32 => Some(Self::PartlyCloudyNight),
            33 => Some(Self::MostlyCloudyNight),
            36 => Some(Self::LightRainShowerNight),
            37 => Some(Self::LightRainShowerAndFlurriesNight),
            38 => Some(Self::LightFlurriesNight),
            39 => Some(Self::ThunderstormNight),
            40 => Some(Self::BlowingSnow),
            41 => Some(Self::FunnelCloud),
            42 => Some(Self::Tornado),
            43 => Some(Self::Windy),
            44 => Some(Self::Smoke),
            45 => Some(Self::DustStorm),
            46 => Some(Self::ThunderstormWithHail),
            47 => Some(Self::ThunderstormWithDust),
            48 => Some(Self::Waterspout),
            _ => None,
        }
    }
}

impl fmt::Display for CurrentConditionIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Self::Sunny => "Sunny",
            Self::MainlySunny => "Mainly Sunny",
            Self::PartlyCloudy => "Partly Cloudy",
            Self::MostlyCloudy => "Mostly Cloudy",
            Self::LightRainShower => "Light Rain Shower",
            Self::LightRainShowerAndFlurries => "Light Rain Shower and Flurries",
            Self::LightFlurries => "Light Flurries",
            Self::Cloudy => "Cloudy",
            Self::Precipitation => "Precipitation",
            Self::RainShower => "Rain Shower",
            Self::Rain => "Rain",
            Self::FreezingRain => "Freezing Rain",
            Self::RainAndFlurries => "Rain and Flurries",
            Self::LightSnow => "Light Snow",
            Self::Flurries => "Flurries",
            Self::HeavySnow => "Heavy Snow",
            Self::Thunderstorm => "Thunderstorm",
            Self::Haze => "Haze",
            Self::Fog => "Fog",
            Self::DriftingSnow => "Drifting Snow",
            Self::IceCrystals => "Ice Crystals",
            Self::IcePellets => "Ice Pellets",
            Self::Drizzle => "Drizzle",
            Self::Clear => "Clear",
            Self::MainlyClear => "Mainly Clear",
            Self::PartlyCloudyNight => "Partly Cloudy",
            Self::MostlyCloudyNight => "Mostly Cloudy",
            Self::LightRainShowerNight => "Light Rain Shower",
            Self::LightRainShowerAndFlurriesNight => "Light Rain Shower and Flurries",
            Self::LightFlurriesNight => "Light Flurries",
            Self::ThunderstormNight => "Thunderstorm",
            Self::BlowingSnow => "Blowing Snow",
            Self::FunnelCloud => "Funnel Cloud",
            Self::Tornado => "Tornado",
            Self::Windy => "Windy",
            Self::Smoke => "Smoke",
            Self::DustStorm => "Dust Storm",
            Self::ThunderstormWithHail => "Thunderstorm with Hail",
            Self::ThunderstormWithDust => "Thunderstorm with Dust",
            Self::Waterspout => "Waterspout",
        };
        write!(f, "{}", description)
    }
}

impl<'de> Deserialize<'de> for CurrentConditionIcon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let code: u8 = s.parse().map_err(serde::de::Error::custom)?;
        Self::from_code(code).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid current condition icon code: {}", code))
        })
    }
}
