use config::{ServerConfig};
use core::db::postgres::PgExecutorAddr;
use rabbitmq::sender::RabbitSenderAddr;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgExecutorAddr,
    pub rabbit_sender: RabbitSenderAddr,
    pub balance_sender: RabbitSenderAddr,
    pub config: ServerConfig,
    pub jwt_public: Vec<u8>,
    pub jwt_private: Vec<u8>,
}

