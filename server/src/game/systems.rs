use bevy_ecs::prelude::*;
use crate::game::components::{Position, Velocity};

pub fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
