use scraper::{Html, Selector};

use chrono::{Timelike, Utc};
use url::Url;

use crate::{Error, Language, sites::SiteInfo};

/// Convenience function to retrieve the latest data.
/// # Basic usage
///
/// ```no_run
#[doc = include_str!("../../examples/deserialize.rs")]
/// ```
pub async fn get_latest<S: SiteInfo>(site: S, language: Language) -> Result<Url, Error> {
    let base = Url::parse(&format!(
        "https://dd.weather.gc.ca/today/citypage_weather/{}/{:02}/",
        site.province().abbr(),
        Utc::now().hour(),
    ))?;

    let mut index_url = base.clone();

    index_url.set_query(Some(&format!(
        "F=0;C=M;O=A;P=*{}_{}*",
        site.code(),
        language.code()
    )));
    let res = reqwest::get(index_url).await?;
    let text = res.text().await?;

    let document = Html::parse_document(&text);
    let selector = Selector::parse("a")?;

    if let Some(link_element) = document
        .select(&selector)
        .find(|e| !e.inner_html().contains("Parent Directory"))
    {
        let href = link_element.attr("href").ok_or_else(|| {
            Error::DataNotFound("Weather data link element missing href attribute".to_string())
        })?;
        Ok(base.join(href)?)
    } else {
        Err(Error::DataNotFound(format!(
            "No weather data link found in directory listing for site {} ({})",
            site.code(),
            language.code()
        )))
    }
}
