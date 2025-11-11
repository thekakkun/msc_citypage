//! Retrieve current weather conditions and 7-days forecast for Canadian cities from the Meteorological Service of Canada (MSC).
//!
//! # Description
//!
//! Provides a [`CityPageStream`] to asynchronously recieive weather and forecast data from MSC as
//! they are published. Initialize by providing a [`site`](sites) and [`Language`] to
//! subscribe to.
//!
//! A convenience function, [`get_latest`] is also provided, and can be used to retrieve the latest
//! data for a specific location and language. This is also called when initially polling the
//! [`CityPageStream`] so weather data is immediately available instead of having to wait for new
//! data to be published.
//!
//! Polling [`CityPageStream`] returns a url to the xml file. Once retrieved, this can be
//! deserialized into [`SiteData`].
//!
//! # Basic usage
//!
//! ```no_run
#![doc = include_str!("../examples/subscribe.rs")]
//! ```
//!
//! # Data source
//!
//! - Environment and Climate Change Canada
//!   - [Meteorological Service of Canada Open Data](https://eccc-msc.github.io/open-data/)

pub mod client;
pub mod error;
pub mod models;
pub mod sites;

pub use client::latest::get_latest;
pub use client::stream::CityPageStream;
pub use error::Error;
pub use models::site_data::SiteData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Français,
}

impl Language {
    /// Returns the ISO 639-1 language code for API requests and internal use.
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Français => "fr",
        }
    }
}
