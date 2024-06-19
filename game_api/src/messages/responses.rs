use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use crate::constraint::username::Username;
use crate::handlers::{self, Handler};
use crate::managers::room::{Room, RoomState};
use crate::managers::statistics::Highscores;

use super::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Response {
    Error(String),
    Login(Result<(), handlers::login::Error>),
    Signup(Result<(), handlers::login::Error>),
    Logout,
    RoomList(Vec<Room>),
    PlayersInRoom(Result<Vec<Username>, handlers::menu::Error>),
    JoinRoom(Result<(), handlers::menu::Error>),
    CreateRoom,
    // PersonalStats(Result<Statistics, handlers::menu::Error>),
    Highscores(Highscores),
    CloseRoom(Result<(), handlers::room_user::Error>),
    StartGame(Result<(), handlers::room_user::Error>),
    RoomState {
        state: RoomState,
        name: String,
        players: Vec<Username>,
    },
    LeaveRoom,
    LeaveGame,
}

impl Response {
    pub fn read_from(reader: &mut impl Read) -> Result<Self, Error> {
        let mut buf_data_len = [0; 4];
        reader.read_exact(&mut buf_data_len)?;
        let data_len = u32::from_le_bytes(buf_data_len);
        let data_len = data_len as usize;

        let mut buf = vec![0; data_len];
        reader.read_exact(&mut buf)?;

        let response = serde_json::from_slice(&buf)?;
        Ok(response)
    }

    pub fn write_to(&self, writer: &mut impl Write) -> Result<(), Error> {
        let json = serde_json::to_vec(self)?;
        let len = json.len() as u32;
        let len_bytes = len.to_le_bytes();
        writer.write_all(&len_bytes)?;
        writer.write_all(&json)?;
        Ok(())
    }

    pub fn new_error(msg: impl ToString) -> Self {
        let msg = msg.to_string();
        Self::Error(msg)
    }
}

pub struct RequestResult {
    pub response: Response,
    pub new_handler: Option<Box<dyn Handler>>,
}

impl RequestResult {
    pub fn new(response: Response, new_handler: impl Handler) -> Self {
        let new_handler = Some(Box::new(new_handler) as Box<dyn Handler>);
        Self {
            response,
            new_handler,
        }
    }

    pub fn without_handler(response: Response) -> Self {
        Self {
            response,
            new_handler: None,
        }
    }

    pub fn new_error(msg: impl ToString) -> Self {
        Self::without_handler(Response::new_error(msg))
    }
}
