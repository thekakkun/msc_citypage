use serde::{Deserialize, Deserializer, de};

use url::Url;

pub fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        T::deserialize(de::value::StringDeserializer::<D::Error>::new(s)).map(Some)
    }
}

macro_rules! deserialize_parseable {
    ($func_name:ident, $type:ty) => {
        pub fn $func_name<'de, D>(deserializer: D) -> Result<Option<$type>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;
            match s {
                Some(s) if s.is_empty() => Ok(None),
                Some(s) => s.parse::<$type>().map(Some).map_err(de::Error::custom),
                None => Ok(None),
            }
        }
    };
}

deserialize_parseable!(deserialize_some_f64, f64);
deserialize_parseable!(deserialize_some_i16, i16);
deserialize_parseable!(deserialize_some_u16, u16);

pub(crate) fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match Url::parse(s.as_str()) {
        Ok(url) => Ok(url),
        Err(_) => Err(de::Error::custom(format!("Invalid url: {}", s))),
    }
}

pub(crate) fn deserialize_some_url<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match Url::parse(s.as_str()) {
        Ok(url) => Ok(Some(url)),
        Err(_) => Err(de::Error::custom(format!("Invalid url: {}", s))),
    }
}
