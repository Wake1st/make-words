use bevy::prelude::*;

mod letter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(letter::plugin);
}
