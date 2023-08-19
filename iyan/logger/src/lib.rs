use log::{Record, Level, Metadata};
use reqwest::Client;
use serde_json::json;

pub struct ElasticsearchLogger {
    client: Client,
    logstash_url: String,
}

impl ElasticsearchLogger {
    pub fn new(logstash_url: &str) -> Self {
        ElasticsearchLogger {
            client: Client::new(),
            logstash_url: logstash_url.to_string(),
        }
    }
}

impl log::Log for ElasticsearchLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info // You can adjust the log level here
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_entry = json!({
                "timestamp": chrono::Utc::now(),
                "level": format!("{:?}", record.level()),
                "message": record.args().to_string(),
            });

            _ = self.client
                .post(&self.logstash_url)
                .header("Content-Type", "application/json") 
                .body(log_entry.to_string()) 
                .send();
        }
    }

    fn flush(&self) {}
}

pub fn init_elasticsearch_logger(logstash_url: &str) -> Result<(), log::SetLoggerError> {
    log::set_max_level(log::LevelFilter::Info); 
    log::set_boxed_logger(Box::new(ElasticsearchLogger::new(logstash_url)))
}
