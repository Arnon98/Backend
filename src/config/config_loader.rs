use anyhow::{Ok, Result};
use super::{config_model::{ASecret, Database, DotEnvyConfig, GSecret, Server}, stage::Stage};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();//Fauce for dowload from system

    let server = Server {
        port:std::env::var("SERVER_PORT")
            .expect("invalid")
            .parse()?,
        
        body_limit:std::env::var("SERVER_BODY_LIMIT")
            .expect("invalid")
            .parse()?,

        timeeout:std::env::var("SERVER_TIMEOUT")
            .expect("invalid")
            .parse()?,
    };

    let database = Database {
    url:std::env::var("DATABASE_URL")
            .expect("Database_invalid"),
    };

    Ok(DotEnvyConfig { server, database})
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();//Fauce for dowload from system

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    // กรณีแปลง String to enum ไม่ได้ จะให้ใช้ default
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_asecret_env() -> Result<ASecret> {
    dotenvy::dotenv().ok();//Fauce for dowload from system

    Ok(ASecret {
        secret:std::env::var("JWT_ASECRET").expect("JWT_ASECRET_INVALID"),
        refresh_secret:std::env::var("JWT_ASECRET_REFRESH").expect("JWT_ASECRET_INVALID"),
    })
}

pub fn get_gsecret_env() -> Result<GSecret> {
    dotenvy::dotenv().ok();//Fauce for dowload from system

    Ok(GSecret {
        secret:std::env::var("JWT_GSECRET").expect("JWT_GSECRET_INVALID"),
        refresh_secret:std::env::var("JWT_GSECRET_REFRESH").expect("JWT_GSECRET_INVALID"),
    })
}