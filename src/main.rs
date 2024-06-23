use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

pub mod bundles;
pub mod component;
pub mod system;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, system::renderer::setup)
        .add_systems(Startup, system::spawn_entities)
        .add_systems(Update, system::renderer::render)
        .add_systems(Update, system::player::input)
        .run();
}
