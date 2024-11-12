use bevy::prelude::*;

pub mod letter;
pub mod word;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((letter::plugin, word::plugin));
}
