use std::collections::HashMap;
use std::iter;

use serde::{Deserialize, Serialize};

use crate::constraint::Username;

use super::room::{Room, RoomID};

pub type Score = f64;
pub type GameID = RoomID;

#[derive(Debug, thiserror::Error)]
pub enum GameErrors {
    #[error("Not Enough Players")]
    NotEnoughPlayers,
}

pub struct GameManager {
    games: HashMap<GameID, Game>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            games: Default::default(),
        }
    }

    pub fn create_game(&mut self, room: &Room) -> Option<&Game> {
        let game = Game::new(room.room_data().room_id, room.users().iter().cloned());
        Some(self.games.entry(game.id).or_insert(game))
    }

    pub fn delete_game(&mut self, id: &GameID) {
        // I don't care if the submission fails
        self.submit_game_results(id).ok();
        self.games.remove(id);
    }

    pub fn game(&self, game_id: &GameID) -> Option<&Game> {
        self.games.get(game_id)
    }

    pub fn game_mut(&mut self, game_id: &GameID) -> Option<&mut Game> {
        self.games.get_mut(game_id)
    }

    pub fn submit_game_results(&mut self, game_id: &GameID) -> Result<(), GameErrors> {
        // let game = self
        //     .games
        //     .remove(game_id)
        //     .ok_or(anyhow!("game {game_id} doesn't exist"))?; // TODO: proper error

        // game.players
        //     .into_iter()
        //     .try_for_each(|(user, data)| self.db.submit_game_data(&user, data))
        Ok(())
    }
}

pub struct Game {
    id: GameID,
    players: HashMap<Username, PlayerData>,
}

impl Game {
    pub fn new(id: RoomID, users: impl Iterator<Item = Username>) -> Self {
        let players = users.zip(iter::repeat_with(PlayerData::default)).collect();

        Self { id, players }
    }

    pub fn id(&self) -> GameID {
        self.id
    }

    pub fn remove_user(&mut self, user: &Username) {
        if let Some(data) = self.players.get_mut(user) {
            // mark as if the user has finished
            // data.left = true;
        }
    }

    pub fn users(&self) -> impl Iterator<Item = &Username> {
        self.players.keys()
    }

    pub fn is_empty(&self) -> bool {
        // self.players.values().all(|data| data.left)
        false
    }

    // NOTE: can be optimized, but I don't really care about performance
    pub fn all_finished(&self) -> bool {
        // because that I'm using (current_question_index - 1) then I'm comparing with `>` instead of `>=`
        // self.players
        //     .values()
        //     .all(|data| data.left || data.current_question_index > self.questions.len())
        false
    }

    pub fn results(&self) -> impl Iterator<Item = (&Username, &PlayerData)> {
        self.players.iter()
    }

    pub fn play_card(
        &self,
        card_id: CardID,
        target_player: Option<Username>,
    ) -> Result<(bool, BallColors), crate::handlers::game::Error> {
        todo!()
    }

    pub fn reveal_card(
        &self,
        guessed_color: BallColors,
    ) -> Result<(bool, BallColors), crate::handlers::game::Error> {
        todo!()
    }

    pub fn draw_card(&self) -> Result<CardID, crate::handlers::game::Error> {
        todo!()
    }
}

// fill
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerData {}

impl PlayerData {
    // pub fn submit_answer(&mut self, correct: bool, answer_time: Duration) {
    //     self.left = false; // if the user left and came back
    //     let old_total = self.correct_answers + self.wrong_answers;
    //     let old_total_time = self.avg_time.as_secs_f64() * old_total as f64;
    //     let total_time = old_total_time + answer_time.as_secs_f64();
    //     let avg_time = total_time / (old_total + 1) as f64;
    //     self.avg_time = Duration::from_secs_f64(avg_time);

    //     if correct {
    //         self.correct_answers += 1;
    //     } else {
    //         self.wrong_answers += 1;
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CardID {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BallColors {
    BLUE,
    ORANGE,
    PURPLE,
    WHITE,
    YELLOW,
    RED,
    THELET,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameData {
    pub players: HashMap<Username, PlayerData>,
    pub played_card: Option<(Username, CardID)>,
    pub drawn_card: Option<Username>,
    //                             guessed color | actuall color
    pub revealed_card: Option<(Username, (BallColors, BallColors))>,
}
