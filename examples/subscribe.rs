use futures_util::stream::StreamExt;
use quick_xml::de::from_str;

use msc_citypage::{CityPageStream, Error, Language, SiteData, sites::Ontario};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    // Initialize the stream for English weather data from Toronto, Ontario
    let mut stream = CityPageStream::new(Ontario::Toronto, Language::English).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(url) => {
                println!("Received: {}", url);

                let response = reqwest::get(url).await?;
                let xml_str = response.text().await?;

                // Deserialize the xml
                match from_str::<SiteData>(&xml_str) {
                    Ok(site_data) => println!("{:?}", site_data),
                    Err(e) => eprintln!("Deserialization error: {}", e),
                }
            }
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }
    Ok(())
}
