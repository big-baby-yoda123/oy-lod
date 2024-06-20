use std::sync::RwLock;

use crate::constraint::Username;
use crate::managers::game::GameID;
use crate::managers::room::RoomID;
use crate::managers::{GameManager, RoomManager};

use super::{GameRequestHandler, Handler, MenuRequestHandler, RoomUserRequestHandler};

pub struct RequestHandlerFactory {
    room_manager: RwLock<RoomManager>,
    game_manager: RwLock<GameManager>,
}

impl RequestHandlerFactory {
    pub fn new() -> Self {
        let room_manager = RoomManager::new();
        let room_manager = RwLock::new(room_manager);
        let game_manager = GameManager::new();
        let game_manager = RwLock::new(game_manager);
        Self {
            room_manager,
            game_manager,
        }
    }

    pub fn create_menu_request_handler(&self, logged_user: Username) -> impl Handler + '_ {
        MenuRequestHandler::new(self, logged_user)
    }

    pub fn create_room_user_request_handler(
        &self,
        user: Username,
        is_admin: bool,
        room_id: RoomID,
    ) -> impl Handler + '_ {
        RoomUserRequestHandler::new(self, user, is_admin, room_id)
    }

    pub fn create_game_request_handler(
        &self,
        user: Username,
        game_id: GameID,
    ) -> impl Handler + '_ {
        GameRequestHandler::new(self, user, game_id)
    }

    pub fn room_manager(&self) -> &RwLock<RoomManager> {
        &self.room_manager
    }

    pub fn game_manager(&self) -> &RwLock<GameManager> {
        &self.game_manager
    }
}
