use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, spawn_entities)
        .add_systems(Update, print_player)
        .run();
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct Name(String);

fn spawn_entities(mut commands: Commands) {
    commands.spawn((Player, Name("Ali".to_string())));
    commands.spawn(Name("Labrat".to_string()));
    commands.spawn(Name("Lumi".to_string()));
}

fn print_player(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.0)
    }
}
