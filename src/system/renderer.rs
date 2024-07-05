use bevy::prelude::*;

use crate::{
    component::{Position, Renderable},
    renderer::{color, terminal::Terminal},
    resources::map::{self, Map},
};

pub fn render(
    objects: Query<(&Position, &Renderable)>,
    mut terminal: ResMut<Terminal>,
    world_map: ResMut<Map>,
) {
    terminal.clear();

    for x in 0..map::MAP_WIDTH as i32 {
        for y in 0..map::MAP_HEIGHT as i32 {
            let code = match world_map[Position { x, y }] {
                map::TileType::Wall => '#' as u8,
                map::TileType::Floor => '.' as u8,
            };
            terminal.put(x, y, code, color::WHITE, color::BLACK);
        }
    }

    for (pos, renderable) in &objects {
        terminal.put(
            pos.x,
            pos.y,
            renderable.glyph as u8,
            renderable.fg,
            renderable.bg,
        )
    }
}
