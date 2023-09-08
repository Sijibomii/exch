use std::io::Error as IoError;

use actix::MailboxError;
use core::ModelError;
use failure::Fail;
use lapin::Error as LapinError;
use deadpool_lapin::{PoolError};


#[derive(Debug, Fail )]
pub enum Error {
    #[fail(display = "exceeded retry limit: {}", _0)]
    RetryLimitError(usize),
    #[fail(display = "{}", _0)]
    ModelError(#[cause] ModelError),
    #[fail(display = "{}", _0)] 
    MailboxError(#[cause] MailboxError),
    #[fail(display = "{}", _0)]
    IoError(#[cause] IoError),
    #[fail(display = "{}", _0)]
    LapinError(#[cause] LapinError),
    #[fail(display =  "unable to get channels to publish: {}", _0)]
    ChannelError(String),
    #[fail(display =  "unable to get channels to publish: {}", _0)]
    RMQError(#[cause] lapin::Error),
    #[fail(display =  "unable to get channels to publish: {}", _0)]
    RMQPoolError(#[cause] PoolError)
}

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Error {
        Error::ModelError(e)
    }
}

impl From<MailboxError> for Error {
    fn from(e: MailboxError) -> Error {
        Error::MailboxError(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::IoError(e)
    }
}

impl From<LapinError> for Error{
    fn from(e: LapinError) -> Error {
        Error::LapinError(e)
    }
}
// From<deadpool::managed::errors::PoolError<lapin::Error>>
impl From<PoolError> for Error{
    fn from(e: PoolError) -> Error {
        Error::RMQPoolError(e)
    }
}