use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct Player {
    pub username: String,
}
