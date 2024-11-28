use bevy::prelude::*;

mod display;
mod interaction;
mod window_size;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((window_size::plugin, interaction::plugin, display::plugin));
}
