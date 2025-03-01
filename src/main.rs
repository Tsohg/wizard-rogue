use bevy::{prelude::*, window::WindowResolution};
use renderer::plugin::*;

pub mod bundles;
pub mod component;
pub mod renderer;
pub mod system;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin()),
            terminal_plugin(),
        ))
        .add_systems(Startup, system::spawn_entities)
        .add_systems(Update, system::renderer::render)
        .add_systems(Update, system::player::input)
        .run();
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin { 
        primary_window: Some(Window {
            title: "Wizard Rogue".to_string(),
            resolution: WindowResolution::new(30. * 8., 30. * 8.),
            ..default()
        }), 
        ..default()
    }
}

fn terminal_plugin() -> TerminalPlugin {
    TerminalBuilder::new()
        .size(30, 30)
        .char_size(8, 8)
        .glyphs("pastiche_8x8.png".to_string(), 16, 16)
        .build()
}
