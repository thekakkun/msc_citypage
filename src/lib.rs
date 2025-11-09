pub mod models;
pub mod stream;

pub use models::site_data::SiteData;
pub use stream::CityPageStream;

pub enum Language {
    English,
    Français,
}
