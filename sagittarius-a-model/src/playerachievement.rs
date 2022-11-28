use serde::{Deserialize, Serialize};

use super::{usermodel::UserId, gamemodel::GameId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerAchievement {
    pub id: String,
    pub tipo: String,
    pub item: String,
    pub timestamp: String,
    pub user_id: UserId,
    pub game_id: GameId
}
