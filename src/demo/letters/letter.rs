use bevy::prelude::*;

use crate::{screens::Screen, AppSet};

use super::word::CreateNewWord;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnLetter>();
    app.add_event::<RemoveLetters>();

    app.add_systems(
        Update,
        (spawn_letter, remove_letters).in_set(AppSet::Update),
    );
}

#[derive(Component, Default)]
pub struct Letter {
    pub value: String,
    pub asset_path: String,
    pub sound_path: String,
    pub sound_duration: f32,
}

impl Clone for Letter {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            asset_path: self.asset_path.clone(),
            sound_path: self.sound_path.clone(),
            sound_duration: self.sound_duration,
        }
    }
}

#[derive(Event)]
pub struct SpawnLetter {
    pub letter: Letter,
    pub position: Vec2,
}

fn spawn_letter(
    mut spawn_event: EventReader<SpawnLetter>,
    mut commands: Commands,
    mut create_word_event: EventWriter<CreateNewWord>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_event.read() {
        let texture: Handle<Image> =
            asset_server.load(format!("images/letters/{}", event.letter.asset_path));
        let mut transform = Transform::from_scale(Vec2::splat(2.0).extend(0.0));
        transform.translation += event.position.extend(0.0);

        let letter_entity = commands
            .spawn((
                Name::new("Letter"),
                event.letter.clone(),
                SpriteBundle {
                    texture,
                    transform,
                    ..Default::default()
                },
                StateScoped(Screen::Gameplay),
            ))
            .id();

        create_word_event.send(CreateNewWord {
            letters: vec![letter_entity],
            links: Vec::new(),
            position: event.position,
        });
    }
}

#[derive(Event)]
pub struct RemoveLetters {
    pub letters: Vec<Entity>,
}

fn remove_letters(mut remove_event: EventReader<RemoveLetters>, mut commands: Commands) {
    for event in remove_event.read() {
        for &letter in event.letters.iter() {
            commands.entity(letter).despawn_recursive();
        }
    }
}
