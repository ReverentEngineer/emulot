use curl::easy::{
    Easy2 as Easy,
    Handler,
    WriteError
};
use url::Url;
use crate::{
    Error,
    ErrorKind,
    de::percent_decode
};
use serde::Deserialize;

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

impl ClientConfig {

    fn common_curl(&self) -> Result<Easy<Collector>, Error> {
        let mut easy = Easy::new(Collector(Vec::new()));
            if std::env::var_os("EMULOT_CURL_VERBOSE").is_some() {
                easy.verbose(true)?;
            }
        Ok(easy)
    }

    fn post<S: AsRef<str>>(&self, path: S) -> Result<(), Error> {
        let mut easy = self.common_curl()?;
        easy.post(true)?;
        let url = &self.url;
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
        easy.perform()?;
        let code = easy.response_code()?;
        let contents = String::from_utf8_lossy(&easy.get_ref().0).to_string();
        match code {
            200 => Ok(()),
            404 => Err(Error::new(ErrorKind::NoSuchEntity, contents)),
            _ => Err(Error::new(ErrorKind::IOError, contents)),
        }
    }

}

impl Default for ClientConfig {

    fn default() -> Self {
        Self {
            url: crate::default_url()
        }
    }

}

pub async fn start(config: ClientConfig, guest: String) -> Result<(), Error> {
    config.post(format!("/guests/start/{guest}"))?;
    Ok(())
}

pub async fn stop(config: ClientConfig, guest: String) -> Result<(), Error> {
    config.post(format!("/guests/stop/{guest}"))?;
    Ok(())
}
