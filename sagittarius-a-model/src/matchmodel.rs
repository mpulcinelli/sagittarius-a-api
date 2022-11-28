use serde::{Deserialize, Serialize};

use super::{usermodel::UserId, gamemodel::GameId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Match {
    pub id: String,
    pub score: i32,
    pub time: i32,
    pub data_jogo: String,
    pub user_id: UserId,
    pub game_id: GameId
}
