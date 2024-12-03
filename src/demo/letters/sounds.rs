use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{audio::SoundEffect, AppSet};

use super::{
    letter::Letter,
    word::{RemoveLettersFromWord, Word},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PlayWordSounds>();

    app.add_systems(
        Update,
        (play_break_sound, play_word_sound, play_delayed_sounds).in_set(AppSet::AudioFeedback),
    );
}

fn play_break_sound(
    mut break_word_event: EventReader<RemoveLettersFromWord>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in break_word_event.read() {
        let source = asset_server.load("audio/sound_effects/mouse_click.ogg");

        commands.spawn((
            AudioBundle {
                source,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    speed: rand::random::<f32>() * 0.1 + 0.9,
                    ..default()
                },
            },
            SoundEffect,
        ));
    }
}

#[derive(Event)]
pub struct PlayWordSounds {
    pub word: Entity,
}

#[derive(Component)]
pub struct DelayedSound {
    pub source: Handle<AudioSource>,
    pub countdown: f32,
}

fn play_word_sound(
    mut play_event: EventReader<PlayWordSounds>,
    words: Query<&Word>,
    letters: Query<&Letter>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in play_event.read() {
        let Ok(word) = words.get(event.word) else {
            continue;
        };

        let mut start_delay = 0.0;
        for &letter_entity in word.letters.iter() {
            let Ok(letter) = letters.get(letter_entity) else {
                continue;
            };

            //	create the delayed sound
            let source = asset_server.load(format!("audio/mouth/{}", letter.sound_path));
            commands.spawn(DelayedSound {
                source,
                countdown: start_delay,
            });

            //	increment the delay
            start_delay += letter.sound_duration;
        }
    }
}

fn play_delayed_sounds(
    mut sounds: Query<(Entity, &mut DelayedSound)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let delta_time = time.delta_seconds();

    for (delay_entity, mut delayed_sound) in sounds.iter_mut() {
        delayed_sound.countdown -= delta_time;

        if delayed_sound.countdown < 0.0 {
            //	play the sound
            commands.spawn((
                AudioBundle {
                    source: delayed_sound.source.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(5.0),
                        speed: rand::random::<f32>() * 0.1 + 0.9,
                        ..default()
                    },
                },
                SoundEffect,
            ));

            //	destroy the delay component
            commands.entity(delay_entity).despawn();
        }
    }
}
