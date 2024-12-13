use bevy::prelude::*;

mod dnd;
pub mod drawer;
mod letters;
mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        movement::plugin,
        dnd::plugin,
        letters::plugin,
        drawer::plugin,
    ));
}
