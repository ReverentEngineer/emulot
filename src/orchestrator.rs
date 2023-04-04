use std::{
    sync::Arc,
    path::{
        Path,
        PathBuf
    }
};
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

    local_storage: PathBuf,

    /// In-memory guests
    guests: CHashMap<String, Arc<Mutex<Guest>>> 

}

impl Orchestrator {

    pub fn new<P: AsRef<Path>>(storage: Arc<ConfigStorage>, local_storage: P) -> Self {
        Self {
            storage,
            local_storage: local_storage.as_ref().to_path_buf(),
            guests: CHashMap::new()
        }
    }

    pub async fn run(&self, id: usize) -> Result<(), Error> {
        let mut guest = self.guests.get(&id.to_string());
        if guest.is_none() {
            let config = self.storage.get(id)?;
            let mut local_storage = self.local_storage.clone();
            local_storage.push(format!("{id}"));
            self.guests.insert(id.to_string(), Arc::new(Mutex::new(Guest::new(config, local_storage))));
            guest = self.guests.get(&id.to_string());
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
