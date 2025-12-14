use std::{
    pin::Pin,
    str::from_utf8,
    task::{Context, Poll},
};

use futures_core::Stream;
use futures_util::{FutureExt, StreamExt};
use lapin::{
    Connection, ConnectionProperties, Consumer, ExchangeKind,
    message::Delivery,
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
};
use url::Url;
use uuid::Uuid;

use crate::{Error, Language, get_latest, sites::SiteInfo};

#[derive(Clone, Debug)]
pub struct CityPageStream<S>
where
    S: SiteInfo + Copy,
{
    pub site: S,
    pub language: Language,
    consumer: Consumer,
    filter_pattern: String,
    latest: Option<Url>,
}

impl<S> CityPageStream<S>
where
    S: SiteInfo + Copy,
{
    pub async fn new(site: S, language: Language) -> Result<Self, Error> {
        let consumer = Self::init_consumer(&site).await?;
        let latest = get_latest(site, language).await.ok();

        Ok(Self {
            consumer,
            site,
            language,
            filter_pattern: format!("{}_{}", &site.code(), language.code()),
            latest,
        })
    }

    async fn init_consumer(site: &S) -> Result<Consumer, Error> {
        let conn = Connection::connect(
            "amqps://anonymous:anonymous@dd.weather.gc.ca/%2f",
            ConnectionProperties::default(),
        )
        .await?;

        let channel = conn.create_channel().await?;
        channel
            .exchange_declare(
                "xpublic",
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    passive: true,
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let queue = channel
            .queue_declare(
                format!(
                    "q_anonymous.rust_msc_citypage.{}.{}",
                    site.code(),
                    Uuid::new_v4()
                )
                .as_str(),
                QueueDeclareOptions {
                    exclusive: true,
                    durable: false,
                    auto_delete: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let routing_key = format!(
            "v02.post.*.WXO-DD.citypage_weather.{}.*",
            site.province().abbr()
        );
        channel
            .queue_bind(
                queue.name().as_str(),
                "xpublic",
                &routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(channel
            .basic_consume(
                queue.name().as_str(),
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?)
    }

    fn handle_delivery(&self, delivery: &Delivery) -> Result<Option<Url>, Error> {
        let body = from_utf8(&delivery.data)?;
        let mut iter = body.split_whitespace();

        let _timestamp_str = iter.next().ok_or_else(|| {
            Error::DataNotFound("AMQP message missing timestamp field".to_string())
        })?;

        let domain = iter
            .next()
            .ok_or_else(|| Error::DataNotFound("AMQP message missing URL field".to_string()))?;
        let mut url = Url::parse(domain)?;

        let path = iter
            .next()
            .ok_or_else(|| Error::DataNotFound("AMQP message missing path field".to_string()))?;

        if path.contains(&self.filter_pattern) {
            url.set_path(path);
            Ok(Some(url))
        } else {
            Ok(None)
        }
    }
}

impl<S> Stream for CityPageStream<S>
where
    S: SiteInfo + Copy + Unpin,
{
    type Item = Result<Url, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(url) = self.latest.take() {
            return Poll::Ready(Some(Ok(url)));
        }

        loop {
            match self.as_mut().get_mut().consumer.next().poll_unpin(cx) {
                Poll::Ready(Some(delivery)) => {
                    let delivery = match delivery {
                        Ok(d) => d,
                        Err(e) => return Poll::Ready(Some(Err(Error::Network(Box::new(e))))),
                    };

                    let result = self.handle_delivery(&delivery).transpose();

                    tokio::spawn(async move {
                        let _ = delivery.ack(BasicAckOptions::default()).await;
                    });

                    if result.is_some() {
                        return Poll::Ready(result);
                    }
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
