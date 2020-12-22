use crate::{config::Config, transport, Result};
use warp::{http::StatusCode, Reply};

#[derive(Serialize, Deserialize, Clone)]
pub struct Status {
    pub current_port: u8,
    pub led_mode: u8,
    pub beeping_mode: u8,
    pub detect_mode: u8,
}

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

pub async fn status_handler(config: Config) -> Result<impl Reply> {
    let status = Status {
        current_port: transport::read_current_port(&config),
        led_mode: 0,
        beeping_mode: 0,
        detect_mode: 0,
    };
    Ok(warp::reply::json(&status))
}

pub async fn switch_port_handler(port: usize, config: Config) -> Result<impl Reply> {
    transport::switch_to_port(&config, port);
    let current_port = transport::read_current_port(&config);
    Ok(current_port.to_string())
}
