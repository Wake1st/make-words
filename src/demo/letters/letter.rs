use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{demo::dnd::cursor::CursorPosition, screens::Screen, AppSet};

use super::word::CreateNewWord;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnLetter>();

    app.add_systems(Startup, load_letters).add_systems(
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
            spawn_letter.in_set(AppSet::Update),
        )
            .chain(),
    );
}

#[derive(Component)]
pub struct Letter {
    pub value: String,
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            value: Default::default(),
            prefixes: Default::default(),
            suffixes: Default::default(),
        }
    }
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
    mut spawn_letter_event: EventWriter<SpawnLetter>,
    mut create_word_event: EventWriter<CreateNewWord>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        let letter = letter_list.letters[0].clone();
        let texture = asset_server.load("images/b.png");
        spawn_letter_event.send(SpawnLetter {
            letter: letter.clone(),
            position: cursor_position.0,
            image: texture.clone(),
        });
        create_word_event.send(CreateNewWord { letter });
    }

    if input.just_pressed(KeyCode::KeyW) {
        let letter = letter_list.letters[1].clone();
        let texture = asset_server.load("images/o.png");
        spawn_letter_event.send(SpawnLetter {
            letter: letter.clone(),
            position: cursor_position.0,
            image: texture.clone(),
        });
        create_word_event.send(CreateNewWord { letter });
    }
}

fn spawn_letter(mut spawn_event: EventReader<SpawnLetter>, mut commands: Commands) {
    for event in spawn_event.read() {
        let mut position = Transform::from_scale(Vec2::splat(2.0).extend(0.0));
        position.translation += event.position.extend(0.0);

        commands.spawn((
            Name::new("Letter"),
            event.letter.clone(),
            SpriteBundle {
                texture: event.image.clone(),
                transform: position,
                ..Default::default()
            },
            StateScoped(Screen::Gameplay),
        ));
    }
}
