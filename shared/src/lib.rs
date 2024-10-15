use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Treasure {
    pub id: String,
    pub location: (f32, f32),
    pub value: u32,
}
