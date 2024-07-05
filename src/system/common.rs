use crate::{
    bundles,
    component::{Name, *},
    renderer::color,
};
use bevy::prelude::*;

pub fn spawn_entities(mut commands: Commands) {
    commands.spawn(bundles::PlayerBundle::default());
    commands.spawn((
        Position { x: 3, y: 3 },
        Renderable {
            glyph: 'g',
            fg: color::RED,
            bg: color::BLACK,
        },
    ));
}

pub fn print_player(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.to_string())
    }
}
