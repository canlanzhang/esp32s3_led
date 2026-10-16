 
use anyhow::Result;

use esp_idf_svc::http::server::{
    Configuration,
    EspHttpServer,
};

use crate::led::Led;

use crate::api;

pub fn start_http_server(
    board_led: Led,
    light_led: Led,
    ip: String,
) -> Result<EspHttpServer<'static>> {
    let mut server =
        EspHttpServer::new(&Configuration::default())?;

    // ==========================
    // 系统状态
    // ==========================

    api::status::register(
        &mut server,
        board_led.clone(),
        light_led.clone(),
        ip,
    )?;

    // ==========================
    // 外接 LED 控制
    // GPIO1
    // ==========================

    api::led::register(
        &mut server,
        light_led,
    )?;

    api::board_led::register(
    &mut server,
    board_led,
)?;

    Ok(server)
}
