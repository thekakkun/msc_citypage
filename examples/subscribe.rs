use futures_util::stream::StreamExt;
use quick_xml::de::from_str;
use std::error::Error;

use msc_citypage::{CityPageStream, SiteData};

/// An example of subscribing to the MSC citypage AMQP stream.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = CityPageStream::new().await?;

    while let Some((datetime, url)) = stream.next().await {
        println!("Received: {}: {}", datetime, url);

        let response = reqwest::get(url).await?;
        let xml_str = response.text().await?;

        match from_str::<SiteData>(&xml_str) {
            Ok(site_data) => println!("{:#?}", site_data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
