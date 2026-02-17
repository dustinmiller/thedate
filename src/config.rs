use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
