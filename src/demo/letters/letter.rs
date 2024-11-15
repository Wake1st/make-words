use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{demo::dnd::cursor::CursorPosition, screens::Screen, AppSet};

use super::{letter_loader::LetterList, word::CreateNewWord};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnLetter>();

    app.add_systems(
        Update,
        (
            (
                spawn_on_input
                    .in_set(AppSet::Update)
                    .run_if(input_just_pressed(KeyCode::KeyQ)),
                spawn_on_input
                    .in_set(AppSet::Update)
                    .run_if(input_just_pressed(KeyCode::KeyW)),
            ),
            spawn_letter,
        )
            .chain(),
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
            sound_duration: self.sound_duration.clone(),
        }
    }
}

#[derive(Event)]
pub struct SpawnLetter {
    letter: Letter,
    position: Vec2,
    image: Handle<Image>,
}

fn spawn_on_input(
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    letter_list: Res<LetterList>,
    cursor_position: Res<CursorPosition>,
    mut spawn_letter: EventWriter<SpawnLetter>,
) {
    let mut letter: Letter = letter_list.letters[1].clone();

    if input.just_pressed(KeyCode::KeyQ) {
        letter = letter_list.letters[1].clone();
    }

    if input.just_pressed(KeyCode::KeyW) {
        letter = letter_list.letters[14].clone();
    }

    let texture: Handle<Image> = asset_server.load(format!("images/letters/{}", letter.asset_path));
    spawn_letter.send(SpawnLetter {
        letter: letter.clone(),
        position: cursor_position.0,
        image: texture.clone(),
    });
}

fn spawn_letter(
    mut spawn_event: EventReader<SpawnLetter>,
    mut commands: Commands,
    mut create_word_event: EventWriter<CreateNewWord>,
) {
    for event in spawn_event.read() {
        let mut position = Transform::from_scale(Vec2::splat(2.0).extend(0.0));
        position.translation += event.position.extend(0.0);

        let letter_entity = commands
            .spawn((
                Name::new("Letter"),
                event.letter.clone(),
                SpriteBundle {
                    texture: event.image.clone(),
                    transform: position,
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
