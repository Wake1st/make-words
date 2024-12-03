use bevy::prelude::*;

pub mod letter;
mod letter_links;
pub mod letter_loader;
pub mod sounds;
pub mod trash;
pub mod window_size;
pub mod word;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        letter::plugin,
        word::plugin,
        letter_links::plugin,
        letter_loader::plugin,
        sounds::plugin,
        trash::plugin,
        window_size::plugin,
    ));
}
