use serde::{Deserialize, Serialize};

use actix::prelude::*;

use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, Connection,
};

use super::errors::Error;

// pub type RabbitClientAddr = Addr<RabbitClient>;

// #[derive(clone)]
pub struct RabbitClient {
    connection: Connection,
    queue_name: String
}


impl RabbitClient {
    pub fn new(connection: Connection, queue_name: String) -> Self {
        Self { connection, queue_name }
    }
}

impl Actor for RabbitClient {
    type Context = Context<Self>;
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
 // for all queues connect. when it get info about trade sent out by elixir deduct balance. if it get cancel incr balance and send msg to db

        // listen to order for when orders are sent out and deduct balance accordingly. if you get cancel: don't do anything

        // listen to responses find cancel-accepted add balance
#[derive(Debug, Deserialize, Serialize)]
struct Response {
    refId: String,
    op: String,
    data: Data,
}
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    seq_num: u32,
    client_id: u32,
    ticker_id: u32,
    order_id: u32,
    side: String,
    price: u32,
    qty: u32,
}


impl RabbitClient {

    async fn start_listening(&self, queue_name: &str) -> Result<(), Error> {
        // Create a channel for message consumption.
        let channel = self.connection.create_channel()
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
                        if queue_name == "response" {
                            if jsn.op == "CLIENT-RESPONSE-CANCELED" {
                                // add price back to balance
                            }
                        }else{
                            //order
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