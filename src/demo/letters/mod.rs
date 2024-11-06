use bevy::prelude::*;

pub mod letter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(letter::plugin);
}
