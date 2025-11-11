use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename = "siteList")]
pub struct SiteList {
    #[serde(rename = "site", default)]
    pub sites: Vec<Site>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Site {
    #[serde(rename = "@code")]
    pub code: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "nameFr")]
    pub name_fr: String,
    #[serde(rename = "provinceCode")]
    pub province: Province,
}
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Province {
    #[serde(alias = "AB", alias = "Alberta")]
    Alberta,

    #[serde(
        alias = "BC",
        alias = "British Columbia",
        alias = "Colombie-Britannique"
    )]
    BritishColumbia,

    #[serde(alias = "MB", alias = "Manitoba")]
    Manitoba,

    #[serde(alias = "NB", alias = "New Brunswick", alias = "Nouveau-Brunswick")]
    NewBrunswick,

    #[serde(
        alias = "NL",
        alias = "Newfoundland and Labrador",
        alias = "Terre-Neuve-et-Labrador"
    )]
    NewfoundlandAndLabrador,

    #[serde(alias = "NS", alias = "Nova Scotia", alias = "Nouvelle-Écosse")]
    NovaScotia,

    #[serde(
        alias = "NT",
        alias = "Northwest Territories",
        alias = "Territoires du Nord-Ouest"
    )]
    NorthwestTerritories,

    #[serde(alias = "NU", alias = "Nunavut")]
    Nunavut,

    #[serde(alias = "ON", alias = "Ontario")]
    Ontario,

    #[serde(
        alias = "PE",
        alias = "Prince Edward Island",
        alias = "Île-du-Prince-Édouard"
    )]
    PrinceEdwardIsland,

    #[serde(alias = "QC", alias = "Quebec", alias = "Québec")]
    Quebec,

    #[serde(alias = "SK", alias = "Saskatchewan")]
    Saskatchewan,

    #[serde(alias = "YT", alias = "Yukon")]
    Yukon,

    #[serde(alias = "HEF")]
    HEF,
}

impl fmt::Display for Province {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Province::Alberta => "Alberta",
            Province::BritishColumbia => "British Columbia",
            Province::Manitoba => "Manitoba",
            Province::NewBrunswick => "New Brunswick",
            Province::NewfoundlandAndLabrador => "Newfoundland and Labrador",
            Province::NovaScotia => "Nova Scotia",
            Province::NorthwestTerritories => "Northwest Territories",
            Province::Nunavut => "Nunavut",
            Province::Ontario => "Ontario",
            Province::PrinceEdwardIsland => "Prince Edward Island",
            Province::Quebec => "Quebec",
            Province::Saskatchewan => "Saskatchewan",
            Province::Yukon => "Yukon",
            Province::HEF => "HEF",
        };
        write!(f, "{}", name)
    }
}

impl Province {
    pub fn abbr(&self) -> &'static str {
        match self {
            Province::Alberta => "AB",
            Province::BritishColumbia => "BC",
            Province::Manitoba => "MB",
            Province::NewBrunswick => "NB",
            Province::NewfoundlandAndLabrador => "NL",
            Province::NovaScotia => "NS",
            Province::NorthwestTerritories => "NT",
            Province::Nunavut => "NU",
            Province::Ontario => "ON",
            Province::PrinceEdwardIsland => "PE",
            Province::Quebec => "QC",
            Province::Saskatchewan => "SK",
            Province::Yukon => "YT",
            Province::HEF => "HEF",
        }
    }
}
