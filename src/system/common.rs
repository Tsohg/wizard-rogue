use crate::{
    bundles,
    component::{Name, *},
};
use bevy::prelude::*;

pub fn spawn_entities(mut commands: Commands) {
    commands.spawn(bundles::PlayerBundle::default());
    commands.spawn((
        Position { x: 3, y: 3 },
        Renderable {
            glyph: 'g',
            fg: Color::RED,
            bg: Color::BLACK,
        },
    ));
}

pub fn print_player(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.to_string())
    }
}
