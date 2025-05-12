use Backend::config::config_loader;
use tracing::{error, info};
#[tokio::main]
async fn main(){
    //Get Log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}",e);
            std::process::exit(1);
        }
    };

    //info!("ENV has been loaded: {:?}", dotenvy_env); | debug
    info!("ENV has been loaded:");
}