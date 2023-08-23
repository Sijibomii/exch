#[macro_use]
extern crate serde_derive;
extern crate bigdecimal;


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub postgres: String, 
    pub server: ServerConfig,
    pub smtp: SmtpConfig
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u64,
    pub mail_sender: String, 
    pub web_client_url: String,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
}
