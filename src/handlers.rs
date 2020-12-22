use crate::{config::Config, transport, Result};
use warp::{http::StatusCode, Reply};

pub async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}

pub async fn led_mode_handler(mode: usize, config: Config) -> Result<impl Reply> {
    transport::led_mode(&config, mode);
    Ok(StatusCode::OK)
}

pub async fn beeping_mode_handler(mode: usize, config: Config) -> Result<impl Reply> {
    transport::beeping_mode(&config, mode);
    Ok(StatusCode::OK)
}

pub async fn detect_mode_handler(mode: usize, config: Config) -> Result<impl Reply> {
    transport::detect_mode(&config, mode);
    Ok(StatusCode::OK)
}

pub async fn current_port_handler(config: Config) -> Result<impl Reply> {
    let current_port = transport::read_current_port(&config);
    Ok(current_port.to_string())
}
pub async fn switch_port_handler(port: usize, config: Config) -> Result<impl Reply> {
    transport::switch_to_port(&config, port);
    let current_port = transport::read_current_port(&config);
    Ok(current_port.to_string())
}
