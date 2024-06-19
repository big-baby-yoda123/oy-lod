use std::borrow::Cow;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::constraint::username::Username;
use crate::managers::room::{RoomData, RoomID, RoomState};
use crate::messages::{Request, RequestInfo, RequestResult, Response};

use super::{Handler, RequestHandlerFactory};

pub struct MenuRequestHandler<'factory> {
    user: Username,
    factory: &'factory RequestHandlerFactory,
}

impl<'factory> Handler for MenuRequestHandler<'factory> {
    fn relevant(&self, request_info: &RequestInfo) -> bool {
        use Request::*;
        matches!(
            request_info.data,
            CreateRoom { .. } | RoomList | JoinRoom(_) | PersonalStats | Highscores | Logout
        )
    }

    fn handle(&mut self, request_info: RequestInfo) -> RequestResult {
        match request_info.data {
            Request::JoinRoom(id) => Ok(self.join_room(id)),
            Request::CreateRoom { name, max_users } => Ok(self.create_room(name, max_users)),
            Request::PersonalStats => {
                let resp = self.get_personal_stats().map_err(|_| Error::NoGamesPlayed);
                let response = Response::PersonalStats(resp);
                let result = RequestResult::without_handler(response);
                Ok(result)
            }
            Request::Highscores => {
                let highscores = self.get_high_scores()?;
                let resp = Response::Highscores(highscores);
                let result = RequestResult::without_handler(resp);
                Ok(result)
            }
            Request::Logout => Ok(self.logout()),
            Request::RoomList => Ok(self.get_rooms()),
            Request::CreateQuestion(question) => self.create_question(question),

            _ => Ok(RequestResult::new_error("Invalid request")),
        }
    }
}

impl<'factory> MenuRequestHandler<'factory> {
    pub fn new(factory: &'factory RequestHandlerFactory, user: Username) -> Self {
        Self { factory, user }
    }

    fn logout(&self) -> RequestResult {
        self.factory
            .login_manager()
            .write()
            .unwrap()
            .logout(&self.user);
        RequestResult::new(
            Response::Logout,
            self.factory.create_login_request_handler(),
        )
    }

    fn get_rooms(&self) -> RequestResult {
        let room_manager = self.factory.room_manager();
        let room_manager_lock = room_manager.read().unwrap();
        let rooms = room_manager_lock.rooms().cloned().collect();
        let response = Response::RoomList(rooms);
        RequestResult::without_handler(response)
    }

    #[allow(unused)]
    fn get_players_in_room(&self, id: RoomID) -> RequestResult {
        let users = self
            .factory
            .room_manager()
            .read()
            .unwrap()
            .room(id)
            .map(|r| r.users().to_vec())
            .ok_or(Error::UnknownRoomID(id));
        RequestResult::without_handler(Response::PlayersInRoom(users))
    }

    // fn get_personal_stats(&self) -> Result<Statistics, db::Error> {
    //     self.factory
    //         .statistics_manager()
    //         .get_user_statistics(&self.user)
    // }

    // fn get_high_scores(&self) -> Result<Highscores, db::Error> {
    //     self.factory.statistics_manager().get_high_scores()
    // }

    fn join_room(&self, id: RoomID) -> RequestResult {
        let mk = Response::JoinRoom;

        let room_manager = self.factory.room_manager();
        let room_manager_lock = room_manager.read().unwrap();
        let Some(room) = room_manager_lock.room(id) else {
            return RequestResult::without_handler(mk(Err(Error::UnknownRoomID(id))));
        };

        if room.is_full() {
            return RequestResult::without_handler(mk(Err(Error::RoomFull)));
        }

        if room.room_data().state == RoomState::InGame {
            return RequestResult::without_handler(mk(Err(Error::RoomInGame)));
        }

        drop(room_manager_lock);
        let mut room_manager_lock = room_manager.write().unwrap();
        let Some(room) = room_manager_lock.room_mut(id) else {
            return RequestResult::without_handler(mk(Err(Error::UnknownRoomID(id))));
        };

        if !room.add_user(self.user.clone()) {
            return RequestResult::without_handler(mk(Err(Error::UserAlreadyInRoom)));
        }

        let resp = mk(Ok(()));
        let handler = self
            .factory
            .create_room_user_request_handler(self.user.clone(), false, id);
        RequestResult::new(resp, handler)
    }

    fn create_room(&self, room_name: Cow<str>, max_users: usize) -> RequestResult {
        let room_data = RoomData::new(room_name, max_users);
        let id = room_data.room_id;
        let room_manager = self.factory.room_manager();
        let mut room_manager_lock = room_manager.write().unwrap();
        room_manager_lock.create_room(self.user.clone(), room_data);
        let resp = Response::CreateRoom;
        let handler = self
            .factory
            .create_room_user_request_handler(self.user.clone(), true, id);
        RequestResult::new(resp, handler)
    }
}

#[derive(Debug, Serialize, Deserialize, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("play a game first")]
    NoGamesPlayed,

    #[error("question already exists")]
    QuestionAlreadyExists,

    #[error("unknown room id {0}")]
    UnknownRoomID(RoomID),

    #[error("room is full")]
    RoomFull,

    #[error("room has started already")]
    RoomInGame,

    #[error("user is already in the room")]
    UserAlreadyInRoom,
}
