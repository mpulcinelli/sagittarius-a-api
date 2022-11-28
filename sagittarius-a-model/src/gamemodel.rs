use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub id: String,
    pub name: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameId {
    pub id: String
}