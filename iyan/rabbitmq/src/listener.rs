use serde::{Deserialize, Serialize};

use actix::prelude::*;
use uuid::Uuid;

use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, Connection, ConnectionProperties,
};

use deadpool_lapin::{Manager, Pool, PoolError};

use super::errors::Error;

use core::{
    wallet::{Wallet, WalletPayload},
    db::postgres::PgExecutorAddr,
};

// pub type RabbitClientAddr = Addr<RabbitClient>;

#[derive()]
pub struct RabbitClient {
    pub pool: Pool,
    pub queue_name: String,
    pub postgres: PgExecutorAddr,
}


impl RabbitClient {
    pub fn new(pool: Pool, queue_name: String, postgres: PgExecutorAddr) -> RabbitClient {
        
        RabbitClient { pool, queue_name, postgres }
    }
}

impl Actor for RabbitClient {
    type Context = SyncContext<Self>;
} 

impl Supervised for RabbitClient {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        match ctx.address().try_send(StartListening) {
            Ok(_) => print!("Restarting"),
            Err(_) => print!("Failed to start polling on restart"),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Stop;

impl Handler<Stop> for RabbitClient {
    type Result = ();

    fn handle(&mut self, _: Stop, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}


#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct StartListening;

// start polling initial event
impl Handler<StartListening> for RabbitClient {
    type Result = Result<(), Error>;

    fn handle(&mut self, _: StartListening, _: &mut Self::Context) -> Self::Result {

        self.start_listening(&self.queue_name);

        Ok(())
    }
}
 
#[derive(Debug, Deserialize, Serialize)]
struct Response {
    refId: Uuid,
    op: String,
    data: Data,
}
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    client_id: u32,
    balance: i64,
    wallet_id: uuid::Uuid
}


impl RabbitClient {

    async fn start_listening(&self, queue_name: &str) -> Result<(), Error> {
        // Create a channel for message consumption.
        let channel = 
        self.pool.get().await.
        map_err(Error::from)
        .unwrap()
        .create_channel()
        .await
        .map_err(Error::from)
        .and_then(|res| {
            return Ok(res);
        }).unwrap_or_else(|e| {
            panic!("failerd to create channel")
        });
        let queue_options = QueueDeclareOptions {
            passive: false,
            durable: false, // Adjust to match the existing queue's durability
            exclusive: false,
            auto_delete: false,
            nowait: false
        };
        // Declare the queue.
        
        channel
            .queue_declare(queue_name, queue_options, FieldTable::default())
            .await?;

        channel
        .queue_bind(
            queue_name,
            "exch",
            queue_name, // Routing key
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

        println!("Listening for messages on queue: {}", queue_name);

        // Create a consumer.
        let mut consumer = channel
            .basic_consume(
                queue_name,
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        
        loop {
            if let Some(delivery) = consumer.next().await {
                match delivery {

                    Ok(delivery) => {
                        let message = delivery.data;
                        let delivery_tag = delivery.delivery_tag;
    
                        // Handle the received message here
                        println!("Received message: {:?}", message);

                        // parse the message with json first
                        let data_string = String::from_utf8_lossy(&message).into_owned();
                        let data_str: &str = data_string.as_str();
                        // Result<Response, serde_json::Error> 
                        let jsn: Response = serde_json::from_str(data_str)
                            .unwrap_or_else(|e| {
                                print!("UNABLE TO DECODE DATA GOTTEN FROM RABBIT. DATA AS STRING ----->>>> {} \n ", data_string);
                                print!("UNABLE TO DECODE DATA GOTTEN FROM RABBIT. DATA AS &str ----->>>> {} \n ", data_str);
                                print!("Error ----->>>> {} \n ", e);
                                panic!("failed to deserialize data from rabbitmq")
                            });
                        if queue_name == "balance" {
                            if jsn.op == "WALLET-BALANCE-CHANGE" {
                                // change balance price back to balance
                                let postgres = self.postgres.clone();
                                match Wallet::find_by_id(jsn.data.wallet_id, &postgres).await {
                                    Ok(wallet) => {
                                        let mut payload = WalletPayload::from(wallet);
                                        payload.balance = Some(jsn.data.balance);
                                        match Wallet::update(jsn.data.wallet_id, payload, &postgres).await {
                                            Ok(p) => {

                                            }
                                            Err(_) => {
                                                println!("unable to update wallet of id: {:?}", jsn.data.wallet_id);
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        println!("unable to find wallet of id: {:?}", jsn.data.wallet_id);
                                    }
                                }
                                
                            }
                        }
                        
                        // Acknowledge the message to remove it from the queue
                        channel.basic_ack(delivery_tag, BasicAckOptions::default()).await?;
                    }
                    Err(e) => eprintln!("Error receiving message: {:?}", e),
                }
            }
        }
    }
}