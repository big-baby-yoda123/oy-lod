pub mod factory;
pub use factory::RequestHandlerFactory;

pub mod game;
pub use game::GameRequestHandler;

pub mod menu;
pub use menu::MenuRequestHandler;

pub mod room_user;
pub use room_user::RoomUserRequestHandler;

use crate::messages::{RequestInfo, RequestResult};

pub trait Handler {
    fn relevant(&self, request_info: &RequestInfo) -> bool;
    fn handle(&mut self, request_info: &RequestInfo) -> RequestResult;
}
