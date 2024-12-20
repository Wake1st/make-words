use bevy::prelude::*;

mod display;
pub mod instructions;
mod interaction;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, display::plugin, instructions::plugin));
}
