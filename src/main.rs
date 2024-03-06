use std::env;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use log::info;

mod api;

static LOG_LEVEL_DEFAULT: &str = "info";
static ENV_FILE_PATH: &str = "./{{project-name}}/.env";
pub const BIND_ADDRESS: &str = "BIND_ADDRESS";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger(LOG_LEVEL_DEFAULT);
    init_env(ENV_FILE_PATH);
    // init_db(std::env::var(DATABASE_URL).expect("DATABASE_URL variable not set").as_str());

    let binding_addr = env::var(BIND_ADDRESS).expect("Error while reading environment variable BIND_ADDRESS");
    info!("starting server on: {}", binding_addr.as_str());
    let _result = start_server(binding_addr.as_str(), AppState::default()).await;
    info!("server terminated");
    Ok(())
}

#[derive(Debug, Clone)]
struct AppState {
    pub app_name: String,
}

impl AppState {
   fn new(app_name: String) -> Self {
       AppState { app_name }
   }
    fn default() -> Self {
        AppState::new("{{project-name}}".to_string())
    }
}

async fn start_server(bind_address: &str, app_state: AppState) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .configure(api::routes::routes)
    })
        .bind(bind_address)?
        .run()
        .await
}

fn init_logger(log_level: &str) {
    if env::var("RUST_LOG").ok().is_none() {
        let log_config = format!("{{project-name}}={},core={},actix_web=info", log_level, log_level);
        env::set_var("RUST_LOG", log_config);
    }
    env_logger::init();
}

fn init_env(env_file: &str) {
    dotenv::from_path(env_file).ok();
    env::vars().for_each(|(k,v)| info!("{}={}", k, v));
}

