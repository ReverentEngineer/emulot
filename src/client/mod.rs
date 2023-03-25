use crate::{
    Error,
    GuestConfig
};

mod config;
pub use config::ClientConfig;
use crate::storage::Labeled;

pub fn start(config: ClientConfig, guest: String) -> Result<(), Error> {
    config.builder()?.endpoint(format!("/guests/start/{guest}"))?.post::<String>(None)?;
    Ok(())
}

pub fn stop(config: ClientConfig, guest: String) -> Result<(), Error> {
    config.builder()?.endpoint(format!("/guests/shutdown/{guest}"))?.post::<String>(None)?;
    Ok(())
}


fn lookup_id(config: &ClientConfig, guest: String) -> Result<isize, Error> {
    config.builder()?
        .endpoint(format!("/guests/lookup/{guest}"))?
        .get()
}

pub fn list(config: ClientConfig) -> Result<(), Error> {
    let guests: Vec<Labeled<isize>> = config.builder()?.endpoint(format!("/guests/list"))?.get()?;
    for guest in guests {
        println!("{0}", guest.label());
    }
    Ok(())
}

pub fn create(config: ClientConfig, guest: String, guest_config: GuestConfig) -> Result<(), Error> {
    config.builder()?.endpoint(format!("/guests/create/{guest}"))?.post(Some(guest_config))?;
    Ok(())
}

pub fn remove(config: ClientConfig, guest: String) -> Result<(), Error> {
    lookup_id(&config, guest)
        .and_then(|id| config.builder()?.endpoint(format!("/guests/remove/{id}"))?.delete())
}
