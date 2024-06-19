use std::collections::HashSet;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::constraint::password::Password;
use crate::constraint::username::Username;

pub struct LoginManager {
    connected: HashSet<Username>,
}

impl LoginManager {
    pub fn new() -> Self {
        Self {
            connected: Default::default(),
        }
    }

    pub fn signup(
        &mut self,
        username: Username,
        password: Password,
        birth_date: NaiveDate,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn login(&mut self, username: Username, password: Password) -> Result<(), Error> {
        todo!()
    }

    pub fn logout(&mut self, username: &Username) {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("user {:?} already connected", .0.as_ref())]
    UserAlreadyConnected(Username),

    #[error("user {:?} already exists", .0.as_ref())]
    UserAlreadyExists(Username),

    #[error("user {:?} doesn't exist", .0.as_ref())]
    UserDoesntExist(Username),

    #[error("wrong password")]
    WrongPassword,
}
