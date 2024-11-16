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
                    .run_if(input_just_pressed(KeyCode::KeyA)),
                spawn_on_input
                    .in_set(AppSet::Update)
                    .run_if(input_just_pressed(KeyCode::KeyB)),
                spawn_on_input
                    .in_set(AppSet::Update)
                    .run_if(input_just_pressed(KeyCode::KeyC)),
                spawn_on_input
                    .in_set(AppSet::Update)
                    .run_if(input_just_pressed(KeyCode::KeyD)),
                spawn_on_input
                    .in_set(AppSet::Update)
                    .run_if(input_just_pressed(KeyCode::KeyO)),
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

    if input.just_pressed(KeyCode::KeyA) {
        letter = letter_list.letters[0].clone();
    }

    if input.just_pressed(KeyCode::KeyB) {
        letter = letter_list.letters[1].clone();
    }

    if input.just_pressed(KeyCode::KeyC) {
        letter = letter_list.letters[2].clone();
    }

    if input.just_pressed(KeyCode::KeyD) {
        letter = letter_list.letters[3].clone();
    }

    if input.just_pressed(KeyCode::KeyE) {
        letter = letter_list.letters[4].clone();
    }

    if input.just_pressed(KeyCode::KeyF) {
        letter = letter_list.letters[5].clone();
    }

    if input.just_pressed(KeyCode::KeyG) {
        letter = letter_list.letters[6].clone();
    }

    if input.just_pressed(KeyCode::KeyH) {
        letter = letter_list.letters[7].clone();
    }

    if input.just_pressed(KeyCode::KeyI) {
        letter = letter_list.letters[8].clone();
    }

    if input.just_pressed(KeyCode::KeyJ) {
        letter = letter_list.letters[9].clone();
    }

    if input.just_pressed(KeyCode::KeyK) {
        letter = letter_list.letters[10].clone();
    }

    if input.just_pressed(KeyCode::KeyL) {
        letter = letter_list.letters[11].clone();
    }

    if input.just_pressed(KeyCode::KeyM) {
        letter = letter_list.letters[12].clone();
    }

    if input.just_pressed(KeyCode::KeyN) {
        letter = letter_list.letters[13].clone();
    }

    if input.just_pressed(KeyCode::KeyO) {
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
