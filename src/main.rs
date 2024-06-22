use bevy::prelude::*;

pub mod component;
pub mod system;

fn main() {
    App::new()
        .add_systems(Startup, system::spawn_entities)
        .add_systems(Update, system::print_player)
        .run();
}
