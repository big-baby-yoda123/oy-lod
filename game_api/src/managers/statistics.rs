use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::constraint::username::Username;

pub type Highscores = Vec<(Username, u32)>;

pub struct StatisticsManager {}

impl StatisticsManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_high_scores(&self) -> Highscores {
        todo!()
    }

    pub fn get_user_statistics(&self, username: &Username) {
        todo!()
    }
}
