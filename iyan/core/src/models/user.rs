use std::convert::From;

use chrono::{prelude::*, Duration};
use futures::Future;
use serde_json::Value;
use uuid::Uuid;

use db::{
    postgres::PgExecutorAddr,
    users::{
        Activate, Delete, DeleteExpired, FindByEmail, FindById, FindByResetToken, Insert, Update, 
    },
};

use models::Error;
use schema::users;

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "users"]
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
    pub last_login_ip: Option<String>,
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
            last_login_ip: None
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
}