use bevy::prelude::*;

mod window_size;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(window_size::plugin);
}
