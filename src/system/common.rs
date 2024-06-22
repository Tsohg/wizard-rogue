use crate::component::{Name, *};
use bevy::prelude::*;

pub fn spawn_entities(mut commands: Commands) {
    commands.spawn((Player, Name::from("Ali")));
    commands.spawn(Name::from("Labrat"));
    commands.spawn(Name::from("Lumi"));
}

pub fn print_player(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.to_string())
    }
}
