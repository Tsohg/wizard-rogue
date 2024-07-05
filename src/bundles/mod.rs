use crate::{component::*, renderer::color};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    renderable: Renderable,
    position: Position,
    player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            renderable: Renderable {
                glyph: '@',
                fg: color::WHITE,
                bg: color::BLACK,
            },
            position: Position { x: 10, y: 10 },
            player: Player {},
        }
    }
}
