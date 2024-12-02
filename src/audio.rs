use bevy::prelude::*;

/// An organizational marker component that should be added to a spawned [`AudioBundle`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
#[derive(Component, Default)]
pub struct Music;

/// An organizational marker component that should be added to a spawned [`AudioBundle`] if it is in the
/// general "sound effect" category (ex: footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category. For example:
#[derive(Component, Default)]
pub struct SoundEffect;
