use anyhow::Result;

use esp_idf_svc::http::server::{Configuration, EspHttpServer};

use crate::{led::Led};

use crate::api;

pub fn start_http_server(
    led: Led,

    ip: String,
) -> Result<EspHttpServer<'static>> {
    let mut server = EspHttpServer::new(&Configuration::default())?;

    api::status::register(&mut server, led.clone(), ip)?;

    api::led::register(&mut server, led)?;

    Ok(server)
}
