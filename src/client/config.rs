use core::marker::PhantomData;
use curl::easy::{
    Easy2 as Easy,
    Handler,
    List,
    WriteError,
    HttpVersion
};
use serde::{
    Deserialize,
    Serialize
};
use url::Url;
use crate::{
    Error,
    ErrorKind,
    de::percent_decode
};


#[derive(Clone, Deserialize)]
pub struct ClientConfig {
    #[serde(deserialize_with = "crate::de::deserialize_url")]
    url: Url
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub struct NeedsEndpoint;
pub struct ReadyToSend;

pub struct RequestBuilder<'a, State> {
    easy: Easy<Collector>,
    config: &'a ClientConfig,
    state: PhantomData<State>
}


impl<'a> RequestBuilder<'a, NeedsEndpoint> {

    pub fn endpoint<S: AsRef<str>>(self, path: S) -> Result<RequestBuilder<'a, ReadyToSend>, Error> {
        let url = &self.config.url;
        let mut easy = self.easy;
        for protocol in url.scheme().split("+") {
            match protocol {
                "http" => (),
                "unix" => {
                    let unix_path = percent_decode(url.path())?;
                    easy.unix_socket(&unix_path)?;
                    easy.url(&format!("http://localhost{}", path.as_ref()))?;
                },
                "tcp" => {
                    let mut new_url = url.clone();
                    new_url.set_scheme("http").unwrap();
                    easy.url(&format!("{new_url}{}", path.as_ref()))?;
                }
                _ => panic!("Invalid protocol")
            }
        }
        Ok(RequestBuilder {
            easy,
            config: self.config,
            state: PhantomData
        })
    }

}

impl<'a> RequestBuilder<'a, ReadyToSend> {

    /// Performs a POST request
    pub fn post<T>(self, data: Option<T>) -> Result<(), Error>
    where
        T: Serialize
    {
        let mut easy = self.easy;
        let mut headers = List::new();
        if let Some(data) = data {
            let data = serde_json::to_vec(&data)?;
            easy.post_fields_copy(&data)?;
            headers.append("Content-Type: application/json")?;
        }
        easy.http_headers(headers)?;
        easy.post(true)?;
        easy.perform()?;
        let code = easy.response_code()?;
        let contents = String::from_utf8_lossy(&easy.get_ref().0).to_string();
        match code {
            200 => Ok(()),
            404 => Err(Error::new(ErrorKind::NoSuchEntity, contents)),
            409 => Err(Error::new(ErrorKind::AlreadyExists, contents)),
            _ => Err(Error::new(ErrorKind::IOError, contents)),
        }
    }

    /// Performs a GET request
    pub fn get<T>(self) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>
    {
        let mut easy = self.easy;
        easy.perform()?;
        let code = easy.response_code()?;
        let contents = String::from_utf8_lossy(&easy.get_ref().0).to_string();
        match code {
            200 => Ok(serde_json::from_slice(&easy.get_ref().0)?),
            404 => Err(Error::new(ErrorKind::NoSuchEntity, contents)),
            409 => Err(Error::new(ErrorKind::AlreadyExists, contents)),
            _ => Err(Error::new(ErrorKind::IOError, contents)),
        }
    }

    /// Performs a DELETE request
    pub fn delete(self) -> Result<(), Error>
    {
        let mut easy = self.easy;
        easy.custom_request("DELETE")?;
        easy.perform()?;
        let code = easy.response_code()?;
        let contents = String::from_utf8_lossy(&easy.get_ref().0).to_string();
        match code {
            200 => Ok(()),
            404 => Err(Error::new(ErrorKind::NoSuchEntity, contents)),
            409 => Err(Error::new(ErrorKind::AlreadyExists, contents)),
            _ => Err(Error::new(ErrorKind::IOError, contents)),
        }
    }




}

impl<'a> ClientConfig {

    pub fn builder(&'a self) -> Result<RequestBuilder<'a, NeedsEndpoint>, Error> {
        let mut easy = Easy::new(Collector(Vec::new()));
        easy.http_version(HttpVersion::V2PriorKnowledge)?;
        if std::env::var_os("EMULOT_CURL_VERBOSE").is_some() {
            easy.verbose(true)?;
        }
        Ok(RequestBuilder {
            easy,
            config: self,
            state: PhantomData
        })
    }
}


impl Default for ClientConfig {

    fn default() -> Self {
        Self {
            url: crate::default_url()
        }
    }

}
