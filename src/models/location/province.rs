use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
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
