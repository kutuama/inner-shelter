use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveCommand {
    pub action: String,
    pub dx: i32,
    pub dy: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionUpdate {
    pub action: String,
    pub username: String,
    pub x: i32,
    pub y: i32,
}
