pub mod components;
pub mod systems;

use bevy_ecs::prelude::*;
use components::*;
use systems::*;
use actix_ws::Session;
use std::collections::HashMap;

pub struct GameState {
    pub world: World,
    pub schedule: Schedule,
    pub sessions: HashMap<String, Session>, // New field to store sessions by username
}

impl GameState {
    pub fn new() -> Self {
        let world = World::new();
        let mut schedule = Schedule::default();

        // Add systems to the schedule
        schedule.add_systems((movement_system,));

        Self {
            world,
            schedule,
            sessions: HashMap::new(), // Initialize the sessions HashMap
        }
    }

    pub fn add_player(&mut self, username: String, session: Session) {
        // Add a new player entity
        self.world.spawn((
            Player { username: username.clone() },
            Position { x: 1.0, y: 1.0 },
            Velocity { x: 0.0, y: 0.0 },
        ));

        // Store the session
        self.sessions.insert(username, session);
    }

    pub fn remove_player(&mut self, username: &str) {
        // Remove the player entity
        let entities: Vec<_> = self
            .world
            .query::<(Entity, &Player)>()
            .iter(&self.world)
            .filter(|(_, player)| player.username == username)
            .map(|(entity, _)| entity)
            .collect();

        for entity in entities {
            self.world.despawn(entity);
        }

        // Remove the session
        self.sessions.remove(username);
    }

    pub fn process_input(&mut self, username: &str, input: serde_json::Value) {
        // Update player's position based on input
        let mut query = self.world.query::<(&Player, &mut Position)>();

        for (player, mut position) in query.iter_mut(&mut self.world) {
            if player.username == username {
                if let Some(action) = input.get("action").and_then(|a| a.as_str()) {
                    match action {
                        "move" => {
                            if let Some(dx) = input.get("dx").and_then(|v| v.as_i64()) {
                                position.x += dx as f64;
                            }
                            if let Some(dy) = input.get("dy").and_then(|v| v.as_i64()) {
                                position.y += dy as f64;
                            }

                            // Ensure the position stays within the grid bounds (1 to 10)
                            position.x = position.x.clamp(1.0, 10.0);
                            position.y = position.y.clamp(1.0, 10.0);
                        }
                        _ => (),
                    }
                }
            }
        }

        // Run systems
        self.schedule.run(&mut self.world);
    }

    pub fn get_positions(&mut self) -> Vec<(String, i32, i32)> {
        // Get positions of all players
        let mut positions = Vec::new();
        let mut query = self.world.query::<(&Player, &Position)>();

        for (player, position) in query.iter(&self.world) {
            // Ensure positions are within the grid bounds and are integers
            let x = position.x.round() as i32;
            let y = position.y.round() as i32;
            positions.push((player.username.clone(), x, y));
        }

        positions
    }
}
