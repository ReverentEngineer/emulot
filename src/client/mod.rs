use crate::{
    Error,
    GuestConfig
};

mod config;
pub use config::ClientConfig;


pub async fn start(config: ClientConfig, guest: String) -> Result<(), Error> {
    config.builder()?.endpoint(format!("/guests/start/{guest}"))?.post::<String>(None)?;
    Ok(())
}

pub async fn stop(config: ClientConfig, guest: String) -> Result<(), Error> {
    config.builder()?.endpoint(format!("/guests/stop/{guest}"))?.post::<String>(None)?;
    Ok(())
}

pub async fn list(config: ClientConfig) -> Result<(), Error> {
    let guests: Vec<String> = config.builder()?.endpoint(format!("/guests/list"))?.get()?;
    for guest in guests {
        println!("{guest}");
    }
    Ok(())
}

pub async fn create(config: ClientConfig, guest: String, guest_config: GuestConfig) -> Result<(), Error> {
    config.builder()?.endpoint(format!("/guests/create/{guest}"))?.post(Some(guest_config))?;
    Ok(())
}
