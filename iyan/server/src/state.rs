use config::{ServerConfig};
use core::db::postgres::PgExecutorAddr;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgExecutorAddr,
    pub config: ServerConfig,
    pub jwt_public: Vec<u8>,
    pub jwt_private: Vec<u8>,
}

