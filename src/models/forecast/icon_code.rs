use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ForecastConditionIcon {
    Sunny = 0,
    FewClouds = 1,
    MixOfSunAndCloud = 2,
    CloudyPeriods = 3,
    IncreasingCloudiness = 4,
    Clearing = 5,
    ChanceOfShowers = 6,
    ChanceOfFlurriesOrRainShowers = 7,
    ChanceOfFlurries = 8,
    ChanceOfThundershowers = 9,
    Cloudy = 10,
    Showers = 12,
    PeriodsOfRain = 13,
    ChanceOfFreezingRain = 14,
    RainShowersOrFlurries = 15,
    Flurries = 16,
    PeriodsOfSnow = 17,
    Blizzard = 18,
    ShowersOrThundershowers = 19,
    MixOfSunAndCloudNight = 22,
    Haze = 23,
    Fog = 24,
    IcePellets = 27,
    Drizzle = 28,
    NotAvailable = 29,
    Clear = 30,
    FewCloudsNight = 31,
    CloudyPeriodsNight = 32,
    CloudyNight = 33,
    IncreasingCloudinessNight = 34,
    ClearingNight = 35,
    ChanceOfShowersNight = 36,
    ChanceOfFlurriesOrShowersNight = 37,
    ChanceOfFlurriesNight = 38,
    ChanceOfThundershowersNight = 39,
    SnowAndBlowingSnow = 40,
    Windy = 43,
    Smoke = 44,
}

impl ForecastConditionIcon {
    pub fn code(&self) -> u8 {
        *self as u8
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Sunny),
            1 => Some(Self::FewClouds),
            2 => Some(Self::MixOfSunAndCloud),
            3 => Some(Self::CloudyPeriods),
            4 => Some(Self::IncreasingCloudiness),
            5 => Some(Self::Clearing),
            6 => Some(Self::ChanceOfShowers),
            7 => Some(Self::ChanceOfFlurriesOrRainShowers),
            8 => Some(Self::ChanceOfFlurries),
            9 => Some(Self::ChanceOfThundershowers),
            10 => Some(Self::Cloudy),
            12 => Some(Self::Showers),
            13 => Some(Self::PeriodsOfRain),
            14 => Some(Self::ChanceOfFreezingRain),
            15 => Some(Self::RainShowersOrFlurries),
            16 => Some(Self::Flurries),
            17 => Some(Self::PeriodsOfSnow),
            18 => Some(Self::Blizzard),
            19 => Some(Self::ShowersOrThundershowers),
            22 => Some(Self::MixOfSunAndCloudNight),
            23 => Some(Self::Haze),
            24 => Some(Self::Fog),
            27 => Some(Self::IcePellets),
            28 => Some(Self::Drizzle),
            29 => Some(Self::NotAvailable),
            30 => Some(Self::Clear),
            31 => Some(Self::FewCloudsNight),
            32 => Some(Self::CloudyPeriodsNight),
            33 => Some(Self::CloudyNight),
            34 => Some(Self::IncreasingCloudinessNight),
            35 => Some(Self::ClearingNight),
            36 => Some(Self::ChanceOfShowersNight),
            37 => Some(Self::ChanceOfFlurriesOrShowersNight),
            38 => Some(Self::ChanceOfFlurriesNight),
            39 => Some(Self::ChanceOfThundershowersNight),
            40 => Some(Self::SnowAndBlowingSnow),
            43 => Some(Self::Windy),
            44 => Some(Self::Smoke),
            _ => None,
        }
    }
}

impl fmt::Display for ForecastConditionIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Self::Sunny => "Sunny",
            Self::FewClouds => "A few clouds",
            Self::MixOfSunAndCloud => "A mix of sun and cloud",
            Self::CloudyPeriods => "Cloudy periods",
            Self::IncreasingCloudiness => "Increasing cloudiness",
            Self::Clearing => "Clearing",
            Self::ChanceOfShowers => "Chance of showers",
            Self::ChanceOfFlurriesOrRainShowers => "Chance of flurries or rain showers",
            Self::ChanceOfFlurries => "Chance of flurries",
            Self::ChanceOfThundershowers => "Chance of thundershowers",
            Self::Cloudy => "Cloudy",
            Self::Showers => "Showers",
            Self::PeriodsOfRain => "Periods of rain",
            Self::ChanceOfFreezingRain => "Chance of freezing rain",
            Self::RainShowersOrFlurries => "Rain showers or flurries",
            Self::Flurries => "Flurries",
            Self::PeriodsOfSnow => "Periods of snow",
            Self::Blizzard => "Blizzard",
            Self::ShowersOrThundershowers => "Showers or thundershowers",
            Self::MixOfSunAndCloudNight => "A mix of sun and cloud",
            Self::Haze => "Haze",
            Self::Fog => "Fog",
            Self::IcePellets => "Ice pellets",
            Self::Drizzle => "Drizzle",
            Self::NotAvailable => "Not available",
            Self::Clear => "Clear",
            Self::FewCloudsNight => "A few clouds",
            Self::CloudyPeriodsNight => "Cloudy periods",
            Self::CloudyNight => "Cloudy",
            Self::IncreasingCloudinessNight => "Increasing cloudiness",
            Self::ClearingNight => "Clearing",
            Self::ChanceOfShowersNight => "Chance of showers",
            Self::ChanceOfFlurriesOrShowersNight => "Chance of flurries or showers",
            Self::ChanceOfFlurriesNight => "Chance of flurries",
            Self::ChanceOfThundershowersNight => "Chance of thundershowers",
            Self::SnowAndBlowingSnow => "Snow and blowing snow",
            Self::Windy => "Windy",
            Self::Smoke => "Smoke",
        };
        write!(f, "{}", description)
    }
}

impl<'de> Deserialize<'de> for ForecastConditionIcon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let code: u8 = s.parse().map_err(serde::de::Error::custom)?;
        Self::from_code(code).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid forecast condition icon code: {}", code))
        })
    }
}
