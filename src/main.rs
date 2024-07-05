use bevy::{prelude::*, window::WindowResolution};
use renderer::plugin::*;
use resources::map;

pub mod bundles;
pub mod component;
pub mod renderer;
pub mod resources;
pub mod system;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin()), terminal_plugin()))
        .insert_resource(resources::map::Map(vec![]))
        .add_systems(Startup, resources::map::draw_map)
        .add_systems(Startup, system::spawn_entities)
        .add_systems(Update, system::renderer::render)
        .add_systems(Update, system::player::input)
        .run();
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Wizard Rogue".to_string(),
            resolution: WindowResolution::new(
                map::MAP_WIDTH as f32 * 8.,
                map::MAP_HEIGHT as f32 * 8.,
            ),
            ..default()
        }),
        ..default()
    }
}

fn terminal_plugin() -> TerminalPlugin {
    TerminalBuilder::new()
        .size(map::MAP_WIDTH, map::MAP_HEIGHT)
        .char_size(8, 8)
        .glyphs("pastiche_8x8.png".to_string(), 16, 16)
        .build()
}
