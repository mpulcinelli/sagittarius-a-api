use serde::{Deserialize, Serialize};

use super::{gamemodel::GameId, usermodel::UserId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemCompra {
    pub id: String,
    pub sku: String,
    pub user_id: UserId,
    pub game_id: GameId,
}

pub struct ItemCompraSKU {
    pub sku: String,
}
