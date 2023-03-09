use crate::{
    error::ErrorKind,
    Error,
    GuestConfig
};
use serde::{Serialize, ser::SerializeMap};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    path::{Path, PathBuf}
};

#[derive(Clone)]
pub struct ConfigStorage {
    #[allow(unused)]
    directory: PathBuf,
    configs: Arc<Mutex<BTreeMap<String, GuestConfig>>>,
}

pub struct Labeled<T> {
    label: String,
    item: T
}

impl<T> Labeled<T> {

    pub fn new<S: AsRef<str>>(label: S, item: T) -> Self {
        Self {
            label: label.as_ref().to_string(),
            item
        }
    }

}

impl<T> Serialize for Labeled<T>
where
    T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(&self.label, &self.item)?;
        map.end()
    }
}

impl ConfigStorage {

    /// Create storage backed by path
    pub async fn new<S: AsRef<str>>(url: S) -> Result<Self, Error> {
       Ok(Self {
            directory: AsRef::<Path>::as_ref(url.as_ref()).to_path_buf(), 
            configs: Arc::new(Mutex::new(BTreeMap::new())),
       })
    }

    pub async fn get(&self, id: &str) -> Option<GuestConfig> {
        let configs = self.configs.lock().unwrap();
        configs.get(id).map(|config| config.clone())
    }

    pub async fn list(&self, begin: Option<usize>, limit: Option<usize>)
        -> Result<Vec<Labeled<GuestConfig>>, Error> {
        let configs = self.configs.lock().unwrap();
        let configs = configs.iter()
            .skip(begin.unwrap_or(0));
        let configs: Vec<_> = if let Some(limit) = limit {
            configs
                .take(limit)
                .map(|(k, v)| Labeled::new(k, v.clone()))
                .collect::<Vec<_>>()
        } else {
            configs
                .map(|(k, v)| Labeled::new(k,  v.clone()))
                .collect::<Vec<_>>()
        };
        Ok(configs)
    }

    pub async fn insert(&self, id: &str, config: GuestConfig) -> Result<(), Error> {
        let mut configs = self.configs.lock().unwrap();
        if !configs.contains_key(id) {
            configs.insert(id.to_string(), config);
            Ok(())
        } else{
            Err(Error::new(ErrorKind::AlreadyExists, format!("A guest config with that ID already exists")))
        }
    }

    pub async fn remove(&self, id: &str) -> Result<(), Error>  {
        let mut configs = self.configs.lock().unwrap();
        configs.remove(id);
        Ok(())
    }

}
