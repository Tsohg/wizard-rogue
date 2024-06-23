use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::component::{Position, Renderable};

// Based on bevy_ascii_terminal example
pub fn setup(mut commands: Commands) {
    let terminal = Terminal::new([80, 60]).with_border(Border::single_line());

    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
}

pub fn render(objects: Query<(&Position, &Renderable)>, mut terminals: Query<&mut Terminal>) {
    for mut terminal in terminals.iter_mut() {
        terminal.clear();
        for (pos, renderable) in &objects {
            terminal.put_char(
                [pos.x, pos.y],
                renderable.glyph.fg(renderable.fg).bg(renderable.bg),
            )
        }
    }
}
