// use std::cell::RefCell;
use actix_rt;
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use lapin::{
    options::*, types::FieldTable, Channel, BasicProperties, ConnectionProperties,
};
use actix::prelude::*;
use uuid::Uuid;
use super::errors::Error;
use lapin::publisher_confirm::PublisherConfirm;
use std::default::Default;
use tokio::runtime::Runtime;
use log::{debug};
use deadpool_lapin::{Pool};

// send events when user logs in

pub type RabbitSenderAddr = Addr<RabbitSender>;

#[derive()]
pub struct RabbitSender {
    pub pool: Pool,
    pub queue_name: String,
    pub channel: Channel
}
 

impl RabbitSender {
    pub fn new(pool: Pool, queue_name: String, channel: Channel) -> Self {
        RabbitSender { pool, queue_name, channel }
    }
}

impl Actor for RabbitSender {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        // Attempt to create a channel when the actor starts
        self.create_channel(&self.queue_name);
    }
} 

impl Supervised for RabbitSender {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        match ctx.address().try_send(StartSending) {
            Ok(_) => print!("Restarting"),
            Err(_) => print!("Failed to start polling on restart"),
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct StartSending;

// start polling initial event
impl Handler<StartSending> for RabbitSender {
    type Result = Result<(), Error>;

    fn handle(&mut self, _: StartSending, _: &mut Self::Context) -> Self::Result {

        let _ = self.create_channel(&self.queue_name);

        Ok(())
    }
}

impl RabbitSender {
    // Function to create a channel and return a Result
    async fn create_channel(&self, queue_name: &str) -> Result<(), Error>  {
        // Create a channel for message consumption.
        let channel = self.pool.get().await.
        map_err(Error::from)
        .unwrap()
        .create_channel()
        .await
        .map_err(Error::from)
        .and_then(|res| {
            return Ok(res);
        }).unwrap_or_else(|e| {
            panic!("failed to create channel")
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
        
        // let mut chan = self.channel.borrow_mut()

        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Stop;

impl Handler<Stop> for RabbitSender {
    type Result = ();

    fn handle(&mut self, _: Stop, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop(); 
    }
}

#[derive(Message,Debug, Deserialize, Serialize)]
#[rtype(result = "Result<Res, Error>")]
pub struct PublishLoginData {
    pub data: Data,
}

pub struct Sender{}

impl Sender{
    pub async fn publish_login(
        payload: Data,
        rabbit: &RabbitSenderAddr, 
    ) -> Result<Res, Error> {
        debug!("rabbits: publishing login heree");
        (*rabbit)
        .send(PublishLoginData{
            data: payload
        })
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }
    // publish login for user with no wallet
    pub async fn publish_login_no_wallet(
        payload: DataNoWallet,
        rabbit: &RabbitSenderAddr, 
    ) -> Result<Res, Error> {

        (*rabbit)
        .send(PublishLoginNoWalletData{
            data: payload
        })
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }


    pub async fn publish_new_token(
        payload: TokenData,
        rabbit: &RabbitSenderAddr, 
    ) -> Result<Res, Error> {

        (*rabbit)
        .send(PublishNewTokenData{
            data: payload
        })
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }

    pub async fn publish_balance(
        payload: BalanceData,
        rabbit: &RabbitSenderAddr, 
    ) -> Result<Res, Error> {
        (*rabbit)
        .send(PublishBalanceData{
            data: payload
        })
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }

    pub async fn wallet_creation(
        payload: WalletCreationData,
        rabbit: &RabbitSenderAddr, 
    ) -> Result<Res, Error> {
        (*rabbit)
        .send(PublishWalletCreationData{
            data: payload
        })
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }

}


pub async fn get_channel(pool: Pool, queue_name: &str) -> Result<Channel, Error> {
    // Create a channel for message consumption.
    let channel = pool.get().await.
    map_err(Error::from)
    .unwrap()
    .create_channel()
    .await
    .map_err(Error::from)
    .and_then(|res| {
        return Ok(res);
    }).unwrap_or_else(|e| {
        panic!("failed to create channel")
    });
    let queue_options = QueueDeclareOptions {
        passive: false,
        durable: false,
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
    
    return Ok(channel)
}

#[derive(Message,Debug, Deserialize, Serialize)]
#[rtype(result = "Result<Res, Error>")]
pub struct PublishLoginNoWalletData {
    pub data: DataNoWallet,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataNoWallet {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub trading_client_id: i64,
    pub last_order_number: i64,
    pub last_seq_num: i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataNoWalletResponse{
    refId: Uuid,
    op: String,
    data: DataNoWallet,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub trading_client_id: i64, 
    pub last_order_number: i64,
    pub last_seq_num: i64,
    pub wallet: Wallet
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet{
    pub id: uuid::Uuid,
    pub balance: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response{
    pub refId: Uuid,
    pub op: String,
    pub data: Data,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Res {
    pub channel: Channel,
    pub queue: String,
    pub data: Vec<u8>,
}


impl Handler<PublishLoginData> for RabbitSender {
 
    type Result = Result<Res, Error>;

    fn handle(
        &mut self,
        PublishLoginData { 
            data
         }: PublishLoginData,
        _: &mut Self::Context,
    ) -> Self::Result {  
        debug!("rabbits: publishing user details email: {} trading_id {}", data.email, data.trading_client_id);
        let channel = self.channel.to_owned();
        let response = Response{
                    refId: Uuid::new_v4(),
                    op:  "USER-LOGIN".to_string(),
                    data: data
                };
        let json_string = serde_json::to_string(&response).expect("Failed to serialize to JSON");
        let bytes = json_string.as_bytes().to_owned();
        let queue = self.queue_name.to_owned();
    
        let result  = Ok(
            Res {
                channel: channel,
                queue: queue,
                data: bytes
            }
        );   

        return result;
    }
}

pub async fn publish(
    channel: &Channel,
    queue_name: String,
    payload: &[u8],
) -> Result<PublisherConfirm, lapin::Error> {
    debug!("rabbits publisher: about to publish to rabbitmq....");
    channel
        .basic_publish(
            "exch",
            &queue_name, // Queue name
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenData {
    pub ticker_id: i64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse{
    refId: Uuid,
    op: String,
    data: TokenData,
}

#[derive(Message,Debug, Deserialize, Serialize)]
#[rtype(result = "Result<Res, Error>")]
pub struct PublishNewTokenData {
    pub data: TokenData,
}


impl Handler<PublishNewTokenData> for RabbitSender {

    type Result = Result<Res, Error>;

    fn handle(
        &mut self,
        PublishNewTokenData { 
            data
         }: PublishNewTokenData,
        _: &mut Self::Context,
    ) -> Self::Result {
        let channel = self.channel.to_owned();
           
        let response = TokenResponse{
                    refId: Uuid::new_v4(),
                    op:  "TOKEN-CREATION".to_string(),
                    data: data
                };
                // Serialize the struct to a JSON string
        let json_string = serde_json::to_string(&response).expect("Failed to serialize to JSON");
        // Convert the JSON string to a &[u8]
        let bytes: Vec<u8> = json_string.as_bytes().to_owned();
        // task::
        let queue = self.queue_name.to_owned();
            
        let result  = Ok(
            Res {
                channel: channel,
                queue: queue,
                data: bytes
            }
        );   

        return result;
        
    }
}

#[derive(Message,Debug, Deserialize, Serialize)]
#[rtype(result = "Result<Res, Error>")]
pub struct PublishBalanceData {
    pub data: BalanceData,
}


impl Handler<PublishBalanceData> for RabbitSender {

    type Result = Result<Res, Error>;

    fn handle(
        &mut self,
        PublishBalanceData { 
            data
         }: PublishBalanceData,
        _: &mut Self::Context,
    ) -> Self::Result {
        let channel = self.channel.to_owned();
           
        let response = BalanceResponse{
                    refId: Uuid::new_v4(),
                    op:  "WALLET-DEPOSIT".to_string(),
                    data: data
                };
                // Serialize the struct to a JSON string
        let json_string = serde_json::to_string(&response).expect("Failed to serialize to JSON");
        // Convert the JSON string to a &[u8]
        let bytes: Vec<u8> = json_string.as_bytes().to_owned();
        // task::
        let queue = self.queue_name.to_owned();
            
        let result  = Ok(
            Res {
                channel: channel,
                queue: queue,
                data: bytes
            }
        );   

        return result;
        
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceData {
    pub client_id: i64,
    pub amount: i64,
    pub wallet_id: uuid::Uuid
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceResponse{
    refId: Uuid,
    op: String,
    data: BalanceData,
}

#[derive(Message,Debug, Deserialize, Serialize)]
#[rtype(result = "Result<Res, Error>")]
pub struct PublishWalletCreationData {
    pub data: WalletCreationData,
}
impl Handler<PublishWalletCreationData> for RabbitSender {

    type Result = Result<Res, Error>;

    fn handle(
        &mut self,
        PublishWalletCreationData { 
            data
         }: PublishWalletCreationData,
        _: &mut Self::Context,
    ) -> Self::Result {

        let channel = self.channel.to_owned();
        let response = WalletCreationResponse{
                    refId: Uuid::new_v4(),
                    op:  "WALLET-CREATED".to_string(),
                    data: data
                };
        // Serialize the struct to a JSON string
        let json_string = serde_json::to_string(&response).expect("Failed to serialize to JSON");
        // Convert the JSON string to a &[u8]
        let bytes = json_string.as_bytes().to_owned();
        
        let queue = self.queue_name.to_owned();
        
        let result  = Ok(
            Res {
                channel: channel,
                queue: queue,
                data: bytes
            }
        );   

        return result;
        
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletCreationData {
    pub client_id: i64,
    pub amount: i64,
    pub wallet_id: uuid::Uuid
}
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletCreationResponse{
    refId: Uuid,
    op: String,
    data: WalletCreationData,
}

// 
impl Handler<PublishLoginNoWalletData> for RabbitSender {

    type Result = Result<Res, Error>;

    fn handle(
        &mut self,
        PublishLoginNoWalletData { 
            data
         }: PublishLoginNoWalletData,
        _: &mut Self::Context,
    ) -> Self::Result {
        debug!("rabbits for no login wallet: starting publishing");
        let channel= self.channel.to_owned();
            
        let response = DataNoWalletResponse{
                    refId: Uuid::new_v4(),
                    op:  "USER-LOGIN-NO-WALLET".to_string(),
                    data: data
                };
        // Serialize the struct to a JSON string
        let json_string = serde_json::to_string(&response).expect("Failed to serialize to JSON");
        // Convert the JSON string to a &[u8]
        let bytes = json_string.as_bytes().to_owned();
        // Publish a message to the queue
        let queue = self.queue_name.to_owned();
    
        let result  = Ok(
            Res {
                channel: channel,
                queue: queue,
                data: bytes
            }
        );   

        return result;
        
    }
}
