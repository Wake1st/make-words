use bevy::prelude::*;

mod drag;
mod drop;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((drag::plugin, drop::plugin));
}
