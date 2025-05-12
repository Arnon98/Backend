#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ASecret {
    pub secret:String,
    pub refresh_secret:String,
}

#[derive(Debug, Clone)]
pub struct GSecret {
    pub secret:String,
    pub refresh_secret:String,
}