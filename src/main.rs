use futures_util::stream::StreamExt;
use std::error::Error;

use msc_citypage::consumer::CityPageStream;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = CityPageStream::new().await?;

    while let Some((datetime, url)) = stream.next().await {
        println!("Received: {} -> {}", datetime, url);
    }

    Ok(())
}
