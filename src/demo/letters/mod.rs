use bevy::prelude::*;

pub mod letter;
mod letter_links;
pub mod word;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((letter::plugin, word::plugin, letter_links::plugin));
}
