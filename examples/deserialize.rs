use msc_citypage::{Error, Language, SiteData, get_latest, sites::Ontario};
use quick_xml::de::from_str;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    if let Ok(url) = get_latest(Ontario::Toronto, Language::English).await {
        let response = reqwest::get(url).await?;
        let xml_str = response.text().await?;

        match from_str::<SiteData>(&xml_str) {
            Ok(site_data) => println!("{:#?}", site_data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
