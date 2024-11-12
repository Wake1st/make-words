use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{demo::dnd::cursor::CursorPosition, screens::Screen, AppSet};

use super::word::CreateNewWord;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnLetter>();

    app.add_systems(Startup, load_letters).add_systems(
        Update,
        (
            spawn_on_input
                .in_set(AppSet::Update)
                .run_if(input_just_pressed(KeyCode::KeyQ)),
            spawn_on_input
                .in_set(AppSet::Update)
                .run_if(input_just_pressed(KeyCode::KeyW)),
        ),
    );
}

#[derive(Component, Default)]
pub struct Letter {
    pub value: String,
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
}

impl Clone for Letter {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            prefixes: self.prefixes.clone(),
            suffixes: self.suffixes.clone(),
        }
    }
}

#[derive(Resource)]
pub struct LetterList {
    pub letters: Vec<Letter>,
}

fn load_letters(mut commands: Commands) {
    commands.insert_resource(LetterList {
        letters: vec![
            Letter {
                value: "b".into(),
                prefixes: vec!["".into()],
                suffixes: vec!["o".into()],
            },
            Letter {
                value: "o".into(),
                prefixes: vec!["b".into()],
                suffixes: vec!["o".into()],
            },
        ],
    });
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
    mut commands: Commands,
    mut create_word_event: EventWriter<CreateNewWord>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        let letter = letter_list.letters[0].clone();
        let texture = asset_server.load("images/b.png");
        let letter_entity = spawn_letter(
            SpawnLetter {
                letter: letter.clone(),
                position: cursor_position.0,
                image: texture.clone(),
            },
            &mut commands,
        );
        create_word_event.send(CreateNewWord {
            letter: letter_entity,
            position: cursor_position.0,
        });
    }

    if input.just_pressed(KeyCode::KeyW) {
        let letter = letter_list.letters[1].clone();
        let texture = asset_server.load("images/o.png");
        let letter_entity = spawn_letter(
            SpawnLetter {
                letter: letter.clone(),
                position: cursor_position.0,
                image: texture.clone(),
            },
            &mut commands,
        );
        create_word_event.send(CreateNewWord {
            letter: letter_entity,
            position: cursor_position.0,
        });
    }
}

fn spawn_letter(spawn: SpawnLetter, commands: &mut Commands) -> Entity {
    let mut position = Transform::from_scale(Vec2::splat(2.0).extend(0.0));
    position.translation += spawn.position.extend(0.0);

    commands
        .spawn((
            Name::new("Letter"),
            spawn.letter.clone(),
            SpriteBundle {
                texture: spawn.image.clone(),
                transform: position,
                ..Default::default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .id()
}
