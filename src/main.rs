extern crate confy;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, warp::Rejection>;

mod config;
mod handlers;
mod packets;
mod transport;

#[tokio::main]
async fn main() {
    use warp::Filter;

    pretty_env_logger::init();

    let config: config::Config =
        confy::load_path("web_kvm.toml").expect("Failed to load configuration!");
    let api_port = config.api_port;

    info!("Service listening on 0.0.0.0 at port {}", api_port);
    info!("KVM address configured as: {}", &config.kvm_address);

    let config_filter = warp::any().map(move || config.clone());

    let health_route = warp::path!("health").and_then(handlers::health_handler);

    let led_mode_route = warp::path!("led_mode" / usize)
        .and(warp::post())
        .and(config_filter.clone())
        .and_then(handlers::led_mode_handler);

    let beep_mode_route = warp::path!("beeping_mode" / usize)
        .and(warp::post())
        .and(config_filter.clone())
        .and_then(handlers::beeping_mode_handler);

    let detect_mode_route = warp::path!("detect_mode" / usize)
        .and(warp::post())
        .and(config_filter.clone())
        .and_then(handlers::detect_mode_handler);

    let switch_port_route = warp::path!("switch" / usize)
        .and(warp::post())
        .and(config_filter.clone())
        .and_then(handlers::switch_port_handler);

    let current_port_route = warp::path!("current")
        .and(warp::get())
        .and(config_filter.clone())
        .and_then(handlers::current_port_handler);

    let routes = health_route
        .or(led_mode_route)
        .or(beep_mode_route)
        .or(detect_mode_route)
        .or(switch_port_route)
        .or(current_port_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], api_port)).await;
}
