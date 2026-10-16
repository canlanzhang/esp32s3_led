
use anyhow::Result;

use esp_idf_hal::io::Write;

use esp_idf_svc::http::server::{EspHttpServer, Method};

use crate::led::Led;

pub fn register(
    server: &mut EspHttpServer<'static>,
    board_led: Led,
    light_led: Led,
    ip: String,
) -> Result<()> {
    server.fn_handler("/api/status", Method::Get, move |req| {
        let board_status =
            if board_led.is_on() { "on" } else { "off" };

        let light_status =
            if light_led.is_on() { "on" } else { "off" };

        let json = format!(
            r#"{{
    "device":"ESP32-S3",
    "wifi":true,
    "ip":"{}",
    "board_led":"{}",
    "light_led":"{}"
}}"#,
            ip,
            board_status,
            light_status,
        );

        let mut response =
            req.into_ok_response()?;

        response.write_all(json.as_bytes())?;

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}