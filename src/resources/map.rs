use std::ops::Index;
use std::vec::Vec;

use bevy::prelude::ResMut;
use bevy::prelude::Resource;

use crate::component::Position;

#[derive(Resource)]
pub struct Map(pub Vec<TileType>);

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

const MAP_HEIGHT: usize = 60;
const MAP_WIDTH: usize = 80;

pub fn draw_map(mut map: ResMut<Map>) {
    map.0 = vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT];
    draw_border(&mut map);
    print!("{:?}", map.0);
}

fn draw_border(map: &mut Map) {
    map.0.splice(0..MAP_WIDTH, [TileType::Wall; MAP_WIDTH]);
}

impl Index<Position> for Map {
    type Output = TileType;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[(index.y as usize * MAP_WIDTH) + index.x as usize]
    }
}
