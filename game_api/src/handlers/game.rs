use serde::{Deserialize, Serialize};
// use tiny_rng::{Rand, Rng};

use crate::constraint::Username;
use crate::managers::game::GameID;
use crate::messages::{Request, RequestInfo, RequestResult, Response};

use super::{Handler, RequestHandlerFactory};

pub struct GameRequestHandler<'factory> {
    game_id: GameID,
    user: Username,
    factory: &'factory RequestHandlerFactory,
}

impl<'factory> Handler for GameRequestHandler<'factory> {
    fn relevant(&self, request_info: &RequestInfo) -> bool {
        use Request::*;
        matches!(
            request_info.data,
            LeaveGame
                | GameResult
                | DrawCard
                | RevealCard { guessed_color: _ }
                | PlayCard {
                    card_id: _,
                    target_player: _
                }
                | GameState
        )
    }

    fn handle(&mut self, request_info: &RequestInfo) -> RequestResult {
        match &request_info.data {
            Request::GameResult => self.game_results(),
            Request::LeaveGame => self.leave_game(),
            Request::DrawCard => self.draw_card(),
            Request::RevealCard { guessed_color } => self.reveal_card(*guessed_color),
            Request::PlayCard {
                card_id,
                target_player,
            } => self.play_card(*card_id, target_player.clone()),
            Request::GameState => self.game_state(),
            _ => RequestResult::new_error("Invalid request"),
        }
    }
}

impl<'factory> GameRequestHandler<'factory> {
    pub fn new(factory: &'factory RequestHandlerFactory, user: Username, game_id: GameID) -> Self {
        Self {
            game_id,
            user,
            factory,
        }
    }

    fn leave_game(&self) -> RequestResult {
        let game_manager = self.factory.game_manager();
        let mut game_manager_lock = game_manager.write().unwrap();
        if let Some(game) = game_manager_lock.game_mut(&self.game_id) {
            game.remove_user(&self.user);

            // no more players are left
            if game.is_empty() {
                game_manager_lock.delete_game(&self.game_id);
                self.factory
                    .room_manager()
                    .write()
                    .unwrap()
                    .delete_room(self.game_id);
            }
        };

        drop(game_manager_lock);

        let resp = Response::LeaveGame;
        let handler = self.factory.create_menu_request_handler(self.user.clone());

        RequestResult::new(resp, handler)
    }

    fn game_results(&self) -> RequestResult {
        // let game_manager = self.factory.game_manager();
        // let game_manager_lock = game_manager.read().unwrap();
        // let Some(game) = game_manager_lock.game(&self.game_id) else {
        //     return RequestResult::without_handler(Response::LeaveGame);
        // };

        // if game.all_finished() {
        //     let mut results: Vec<_> = game
        //         .results()
        //         .map(|(user, data)| {
        //             PlayerResults::new(
        //                 user.clone(),
        //                 data.correct_answers,
        //                 data.wrong_answers,
        //                 data.avg_time,
        //             )
        //         })
        //         .collect();
        //     results.sort_by(|res1, res2| res1.score.total_cmp(&res2.score).reverse());

        //     drop(game_manager_lock);
        //     self.leave_game();

        //     let resp = Response::GameResult(results);
        //     let handler = self.factory.create_menu_request_handler(self.user.clone());
        //     RequestResult::new(resp, handler)
        // } else {
        //     let resp = Response::GameResult(vec![]);
        //     RequestResult::without_handler(resp)
        // }
        RequestResult::new_error("Not impl")
    }

    fn draw_card(&self) -> RequestResult {
        let game_manager = self.factory.game_manager();
        let mut game_manager_lock = game_manager.write().unwrap();
        let Some(game) = game_manager_lock.game_mut(&self.game_id) else {
            let resp = Response::DrawCard(Err(Error::UnknownGameID(self.game_id)));
            return RequestResult::without_handler(resp);
        };

        let card_result = game.draw_card();

        let resp = Response::DrawCard(card_result);
        RequestResult::without_handler(resp)
    }

    fn reveal_card(&self, guessed_color: crate::managers::game::BallColors) -> RequestResult {
        let game_manager = self.factory.game_manager();
        let mut game_manager_lock = game_manager.write().unwrap();
        let Some(game) = game_manager_lock.game_mut(&self.game_id) else {
            let resp = Response::RevealCard(Err(Error::UnknownGameID(self.game_id)));
            return RequestResult::without_handler(resp);
        };

        let card_result = game.reveal_card(guessed_color);

        let resp = Response::RevealCard(card_result);
        RequestResult::without_handler(resp)
    }

    fn play_card(
        &self,
        card_id: crate::managers::game::CardID,
        target_player: Option<Username>,
    ) -> RequestResult {
        let game_manager = self.factory.game_manager();
        let mut game_manager_lock = game_manager.write().unwrap();
        let Some(game) = game_manager_lock.game_mut(&self.game_id) else {
            let resp = Response::RevealCard(Err(Error::UnknownGameID(self.game_id)));
            return RequestResult::without_handler(resp);
        };

        let card_result = game.play_card(card_id, target_player);

        let resp = Response::RevealCard(card_result);
        RequestResult::without_handler(resp)
    }

    fn game_state(&self) -> RequestResult {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("unknown game id {0}")]
    UnknownGameID(GameID),
}
