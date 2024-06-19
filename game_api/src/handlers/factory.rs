use std::sync::RwLock;

use crate::constraint::username::Username;
use crate::managers::game::GameID;
use crate::managers::room::RoomID;
use crate::managers::{GameManager, LoginManager, RoomManager, StatisticsManager};

use super::{
    GameRequestHandler, Handler, LoginRequestHandler, MenuRequestHandler, RoomUserRequestHandler,
};

pub struct RequestHandlerFactory {
    login_manager: RwLock<LoginManager>,
    room_manager: RwLock<RoomManager>,
    statistics_manager: StatisticsManager,
    game_manager: RwLock<GameManager>,
}

impl RequestHandlerFactory {
    pub fn new() -> Self {
        let login_manager = LoginManager::new();
        let login_manager = RwLock::new(login_manager);
        let room_manager = RoomManager::new();
        let room_manager = RwLock::new(room_manager);
        let statistics_manager = StatisticsManager::new();
        let game_manager = GameManager::new();
        let game_manager = RwLock::new(game_manager);
        Self {
            login_manager,
            room_manager,
            statistics_manager,
            game_manager,
        }
    }

    pub fn create_login_request_handler(&self) -> impl Handler {
        LoginRequestHandler::new(self)
    }

    pub fn create_menu_request_handler(&self, logged_user: Username) -> impl Handler {
        MenuRequestHandler::new(self, logged_user)
    }

    pub fn create_room_user_request_handler(
        &self,
        user: Username,
        is_admin: bool,
        room_id: RoomID,
    ) -> impl Handler {
        RoomUserRequestHandler::new(self, user, is_admin, room_id)
    }

    pub fn create_game_request_handler(&self, user: Username, game_id: GameID) -> impl Handler {
        GameRequestHandler::new(self, user, game_id)
    }

    pub fn login_manager(&self) -> &RwLock<LoginManager> {
        &self.login_manager
    }

    pub fn room_manager(&self) -> &RwLock<RoomManager> {
        &self.room_manager
    }

    pub fn statistics_manager(&self) -> &StatisticsManager {
        &self.statistics_manager
    }

    pub fn game_manager(&self) -> &RwLock<GameManager> {
        &self.game_manager
    }
}
