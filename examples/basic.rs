use futures_util::stream::StreamExt;
use std::{error::Error, io::Cursor};
use url::Url;
use xsd_parser::quick_xml::{DeserializeSync, IoReader, XmlReader};

use msc_citypage::{consumer::CityPageStream, schemas::SiteData};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = CityPageStream::new().await?;

    while let Some((datetime, url)) = stream.next().await {
        println!("Received: {} -> {}", datetime, url);
        let site_data = get_site_data(url).await?;

        println!("{:?}", site_data)
    }

    Ok(())
}

async fn get_site_data(url: Url) -> Result<SiteData, Box<dyn Error>> {
    let xml = reqwest::get(url).await?.bytes().await?;
    let cursor = Cursor::new(xml);
    let mut reader = IoReader::new(cursor).with_error_info();

    Ok(SiteData::deserialize(&mut reader)?)
}
