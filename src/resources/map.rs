use std::ops::Index;
use std::ops::IndexMut;
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

pub const MAP_HEIGHT: usize = 60;
pub const MAP_WIDTH: usize = 80;

pub fn draw_map(mut map: ResMut<Map>) {
    map.0 = vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT];
    draw_square(&mut map);
}

fn draw_square(map: &mut Map) {
    for x in 0..MAP_WIDTH as i32 {
        for y in 0..MAP_HEIGHT as i32 {
            if x == 0 || y == 0 || x == (MAP_WIDTH - 1) as i32 || y == (MAP_HEIGHT - 1) as i32 {
                map[Position { x, y }] = TileType::Wall;
            }
        }
    }
}

impl Index<Position> for Map {
    type Output = TileType;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[(index.y * MAP_WIDTH as i32 + index.x) as usize]
    }
}

impl IndexMut<Position> for Map {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.0[(index.y * MAP_WIDTH as i32 + index.x) as usize]
    }
}
