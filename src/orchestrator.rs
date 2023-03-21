use std::sync::Arc;
use tokio::sync::Mutex;
use chashmap::CHashMap;
use crate::{
    Error,
    ErrorKind,
    Guest,
    storage::ConfigStorage
};

pub struct Orchestrator {
   
    /// On-disk storage
    storage: Arc<ConfigStorage>,

    /// In-memory guests
    guests: CHashMap<String, Arc<Mutex<Guest>>> 

}

impl Orchestrator {

    pub fn new(storage: Arc<ConfigStorage>) -> Self {
        Self {
            storage,
            guests: CHashMap::new()
        }
    }

    pub async fn run(&self, id: usize) -> Result<(), Error> {
        let mut guest = self.guests.get(&id.to_string());
        if guest.is_none() {
            if let Some(config) = self.storage.get(id)? {
                self.guests.insert(id.to_string(), Arc::new(Mutex::new(config.into()))); 
                guest = self.guests.get(&id.to_string());
            }
        }

        if let Some(guest) = guest {
            let mut guard = guest.lock().await;
            guard.run().await?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NoSuchEntity, "Guest not found"))
        }
    }

    pub async fn shutdown(&self, id: &str) -> Result<(), Error> {
        if let Some(guest) = self.guests.get(&id.to_string()) {
            let mut guard = guest.lock().await;
            guard.shutdown().await?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NoSuchEntity, "Guest not found"))
        }
    }

}
