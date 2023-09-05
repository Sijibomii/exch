use std::convert::From;

use chrono::{prelude::*, Duration};
use serde_json::Value;
use uuid::Uuid;

use super::super::db::{
    postgres::PgExecutorAddr,
    users::{
        Activate, Delete, DeleteExpired, FindByEmail, FindById, FindByResetToken, Insert, Update, 
    },
};
use super::Error;
use super::super::schema::users;

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct UserPayload {
    pub email: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_verified: Option<bool>,
    pub verification_token: Option<Uuid>,
    pub verification_token_expires_at: Option<DateTime<Utc>>,
    pub reset_token: Option<Option<Uuid>>,
    pub reset_token_expires_at: Option<Option<DateTime<Utc>>>,
    pub last_login_time: Option<Option<DateTime<Utc>>>,
    pub last_login_ip: Option<Option<String>>,
    pub trading_client_id: Option<i64>,
    pub last_order_id: Option<i64>,
    pub last_seq_num: Option<i64>,
}

impl UserPayload {
    pub fn new() -> Self {
        UserPayload {
            email: None,
            password: None,
            salt: None,
            created_at: None,
            updated_at: None,
            is_verified: None,
            verification_token: None,
            verification_token_expires_at: None,
            reset_token: None,
            reset_token_expires_at: None,
            last_login_time: None,
            last_login_ip: None,
            trading_client_id: None,
            last_order_id: None,
            last_seq_num: None,
        }
    }

    pub fn set_created_at(&mut self) {
        self.created_at = Some(Utc::now());
    }

    pub fn set_updated_at(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    pub fn set_verification_token(&mut self) {
        self.is_verified = Some(false);
        self.verification_token = Some(Uuid::new_v4());
        self.verification_token_expires_at = Some(Utc::now() + Duration::days(1));
    }

    pub fn set_reset_token(&mut self) {
        self.reset_token = Some(Some(Uuid::new_v4()));
        self.reset_token_expires_at = Some(Some(Utc::now() + Duration::days(1)));
    }

    pub fn set_last_login_time(&mut self) {
        self.last_login_time = Some(Some(Utc::now()));
    }

}


#[derive(Identifiable, Queryable, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_verified: bool,
    pub verification_token: Uuid,
    pub verification_token_expires_at: DateTime<Utc>,
    pub reset_token: Option<Uuid>,
    pub reset_token_expires_at: Option<DateTime<Utc>>,
    pub last_login_time: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,
    pub trading_client_id: i64,
    pub last_order_id: i64,
    pub last_seq_num: i64
}

impl From<User> for UserPayload {
    fn from(user: User) -> Self {
        UserPayload {
            email: Some(user.email),
            password: Some(user.password),
            salt: Some(user.salt),
            created_at: Some(user.created_at),
            updated_at: Some(user.updated_at),
            is_verified: Some(user.is_verified),
            verification_token: Some(user.verification_token),
            verification_token_expires_at: Some(user.verification_token_expires_at),
            reset_token: Some(user.reset_token),
            reset_token_expires_at: Some(user.reset_token_expires_at),
            last_login_time: Some(user.last_login_time),
            last_login_ip: Some(user.last_login_ip),
            trading_client_id: Some(user.trading_client_id),
            last_order_id: Some(user.last_order_id),
            last_seq_num: Some(user.last_seq_num),
        }
    }
}

impl User {
    pub async fn insert(
        mut payload: UserPayload,
        postgres: &PgExecutorAddr,
    ) ->  Result<User, Error> {
        payload.set_created_at();
        payload.set_updated_at(); 
        payload.set_verification_token();
        
        let sum_result = (*postgres)
        .send(Insert(payload))
        .await
        .map_err(|e| {
            eprintln!("Encountered mailbox error: {:?}", e);
            Error::from(e)
        });

    match sum_result {
        Ok(res) => 
        match res {
            Ok(user) => Ok(user),
            Err(e) => Err(Error::from(e)),
        }
        ,
        Err(e) => Err(e),
        }
    }

    pub async fn update(
        id: Uuid,
        mut payload: UserPayload,
        postgres: &PgExecutorAddr,
    ) ->  Result<User, Error> {
        payload.set_updated_at();

        let sum_result = (*postgres)
            .send(Update { id, payload })
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_reset_token(
        token: Uuid,
        postgres: &PgExecutorAddr,
    ) ->  Result<User, Error> {
        let sum_result=(*postgres)
            .send(FindByResetToken(token))
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_email(
        email: String,
        postgres: &PgExecutorAddr,
    ) ->  Result<User, Error> {
        let sum_result= (*postgres)
            .send(FindByEmail(email))
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_id(
        id: Uuid,
        postgres: &PgExecutorAddr,
    ) ->  Result<User, Error> {
        let sum_result = (*postgres)
            .send(FindById(id))
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }

    pub async fn activate(
        token: Uuid,
        postgres: &PgExecutorAddr,
    ) ->  Result<User, Error> {
        // send activate message to pgexecutor
        let sum_result= (*postgres)
            .send(Activate(token))
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }

    pub async fn delete(id: Uuid, postgres: &PgExecutorAddr) -> Result<usize, Error> {
        let sum_result = (*postgres)
            .send(Delete(id))
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }
 
    pub async fn delete_expired(
        email: String,
        postgres: &PgExecutorAddr,
    ) -> Result<usize, Error> {
        let sum_result = (*postgres)
            .send(DeleteExpired(email))
            .await
            .map_err(|e| {
                eprintln!("Encountered mailbox error: {:?}", e);
                Error::from(e)
            });

        match sum_result {
            Ok(res) => 
            match res {
                Ok(user) => Ok(user),
                Err(e) => Err(Error::from(e)),
            }
            ,
            Err(e) => Err(e),
        }
    }

    pub fn export(&self) -> Value {
        json!({
            "id": self.id,
            "email": self.email,
            "created_at": self.created_at.timestamp(),
            "updated_at": self.updated_at.timestamp(),
        })
    }
}