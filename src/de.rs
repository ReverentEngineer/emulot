use core::fmt;
use std::path::PathBuf;
use serde::{
    Deserializer,
    de,
    de::Visitor
};
use url::Url;
use crate::{
    Error,
    ErrorKind
};

pub fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>
{
    struct PathVisitor;

    impl<'de> Visitor<'de> for PathVisitor {

        type Value = PathBuf;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("url")
        }

        fn visit_str<E>(self, value: &str) -> Result<PathBuf, E>
        where
            E: de::Error,
        {
            Ok(value.into())
        }

    }


    deserializer.deserialize_str(PathVisitor)
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<url::Url, D::Error>
where
    D: Deserializer<'de>
{
    struct UrlVisitor;

    impl<'de> Visitor<'de> for UrlVisitor {

        type Value = Url;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("url")
        }

        fn visit_str<E>(self, value: &str) -> Result<Url, E>
        where
            E: de::Error,
        {
            Url::parse(value)
                .map_err(|err| E::custom(format!("{err}")))
        }

    }


    deserializer.deserialize_str(UrlVisitor)
}

pub fn percent_decode(path: &str) -> Result<String, Error> {
    let mut result = String::new();
    let mut chars = path.chars();
    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let first = chars.next()
                    .ok_or(Error::new(ErrorKind::EncodingError, format!("Expect encoding chars")))?;
                let second = chars.next()
                    .ok_or(Error::new(ErrorKind::EncodingError, format!("Expect encoding chars")))?;

                match (first, second) {
                    ('2', '0') => result.push(' '),
                    _ => return Err(Error::new(ErrorKind::EncodingError, format!("Unsupported encoding")))
                }
            },
            c => result.push(c)
        }
    }
    Ok(result)
}
