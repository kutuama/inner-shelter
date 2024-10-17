pub mod components;
pub mod systems;

use bevy_ecs::prelude::*;
use components::*;
use systems::*;

pub struct GameState {
    pub world: World,
    pub schedule: Schedule,
}

impl GameState {
    pub fn new() -> Self {
        let world = World::new();
        let mut schedule = Schedule::default();

        // Add systems to the schedule
        schedule.add_systems((movement_system,));

        Self { world, schedule }
    }

    pub fn add_player(&mut self, username: String) {
        // Add a new player entity
        self.world.spawn((
            Player { username },
            Position { x: 0.0, y: 0.0 },
            Velocity { x: 0.0, y: 0.0 },
        ));
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
    }

    pub fn process_input(&mut self, username: &str, input: serde_json::Value) {
        // Update player's velocity based on input
        let mut query = self.world.query::<(&Player, &mut Velocity)>();

        for (player, mut velocity) in query.iter_mut(&mut self.world) {
            if player.username == username {
                if let Some(action) = input.get("action").and_then(|a| a.as_str()) {
                    match action {
                        "move" => {
                            if let Some(dx) = input.get("dx").and_then(|v| v.as_f64()) {
                                velocity.x = dx;
                            }
                            if let Some(dy) = input.get("dy").and_then(|v| v.as_f64()) {
                                velocity.y = dy;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        // Run systems
        self.schedule.run(&mut self.world);
    }

    pub fn get_positions(&mut self) -> Vec<(String, f64, f64)> {
        let mut positions = Vec::new();
        let mut query = self.world.query::<(&Player, &Position)>();

        for (player, position) in query.iter(&mut self.world) {
            positions.push((player.username.clone(), position.x, position.y));
        }

        positions
    }
}
