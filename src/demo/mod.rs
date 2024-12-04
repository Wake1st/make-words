use bevy::prelude::*;

mod dnd;
pub mod drawer;
mod letters;
pub mod level;
mod movement;
pub mod player;
pub mod volume_control;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        movement::plugin,
        player::plugin,
        level::plugin,
        dnd::plugin,
        letters::plugin,
        drawer::plugin,
        volume_control::plugin,
    ));
}
