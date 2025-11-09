mod date_stamp;
mod period;
pub mod serde_helpers;

pub use date_stamp::DateStamp;
pub use period::Period;
pub use serde_helpers::{
    deserialize_some_f64, deserialize_some_i16, deserialize_some_u16, empty_string_as_none,
};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum DayName {
    #[serde(rename = "Today", alias = "Aujourd'hui")]
    Today,

    #[serde(
        rename = "Tonight",
        alias = "Ce soir et cette nuit",
        alias = "ce soir et cette nuit"
    )]
    Tonight,

    #[serde(rename = "Monday", alias = "Lundi", alias = "lundi")]
    Monday,

    #[serde(
        rename = "Monday night",
        alias = "Lundi soir et nuit",
        alias = "lundi soir et nuit"
    )]
    MondayNight,

    #[serde(rename = "Tuesday", alias = "Mardi", alias = "mardi")]
    Tuesday,

    #[serde(
        rename = "Tuesday night",
        alias = "Mardi soir et nuit",
        alias = "mardi soir et nuit"
    )]
    TuesdayNight,

    #[serde(rename = "Wednesday", alias = "Mercredi", alias = "mercredi")]
    Wednesday,

    #[serde(
        rename = "Wednesday night",
        alias = "Mercredi soir et nuit",
        alias = "mercredi soir et nuit"
    )]
    WednesdayNight,

    #[serde(rename = "Thursday", alias = "Jeudi", alias = "jeudi")]
    Thursday,

    #[serde(
        rename = "Thursday night",
        alias = "Jeudi soir et nuit",
        alias = "jeudi soir et nuit"
    )]
    ThursdayNight,

    #[serde(rename = "Friday", alias = "Vendredi", alias = "vendredi")]
    Friday,

    #[serde(
        rename = "Friday night",
        alias = "Vendredi soir et nuit",
        alias = "vendredi soir et nuit"
    )]
    FridayNight,

    #[serde(rename = "Saturday", alias = "Samedi", alias = "samedi")]
    Saturday,

    #[serde(
        rename = "Saturday night",
        alias = "Samedi soir et nuit",
        alias = "samedi soir et nuit"
    )]
    SaturdayNight,

    #[serde(rename = "Sunday", alias = "Dimanche", alias = "dimanche")]
    Sunday,

    #[serde(
        rename = "Sunday night",
        alias = "Dimanche soir et nuit",
        alias = "dimanche soir et nuit"
    )]
    SundayNight,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Format {
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "png")]
    Png,
}
