use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    demo::{
        dnd::{cursor::CursorPosition, drag::Draggable, drop::DropZone},
        movement::ScreenWrap,
    },
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_letters);
    app.add_systems(
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

fn spawn_on_input(
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    letter_list: Res<LetterList>,
    cursor_position: Res<CursorPosition>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        let letter = letter_list.letters[0].clone();
        let texture = asset_server.load("images/b.png");
        spawn_letter(
            letter,
            texture,
            cursor_position.0.extend(0.0),
            &mut commands,
        );
    }

    if input.just_pressed(KeyCode::KeyW) {
        let letter = letter_list.letters[1].clone();
        let texture = asset_server.load("images/o.png");
        spawn_letter(
            letter,
            texture,
            cursor_position.0.extend(0.0),
            &mut commands,
        );
    }
}

fn spawn_letter(
    letter: Letter,
    texture: Handle<Image>,
    cursor_position: Vec3,
    commands: &mut Commands,
) {
    let mut position = Transform::from_scale(Vec2::splat(2.0).extend(0.0));
    position.translation += cursor_position;

    commands.spawn((
        Name::new("Letter"),
        letter,
        SpriteBundle {
            texture: texture.clone(),
            transform: position,
            ..Default::default()
        },
        Draggable {
            size: Vec2::splat(256.0),
        },
        DropZone {
            size: Vec2::splat(256.0),
            ..default()
        },
        ScreenWrap,
        StateScoped(Screen::Gameplay),
    ));
}
