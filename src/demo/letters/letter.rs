use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    demo::{
        dnd::{drag::Draggable, drop::DropZone},
        movement::ScreenWrap,
    },
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_letters);
    app.add_systems(
        Update,
        spawn_letter
            .in_set(AppSet::Update)
            .run_if(input_just_pressed(MouseButton::Right)),
    );
}

#[derive(Component)]
pub struct Letter {
    pub value: String,
    pub prefixes: Vec<String>,
    pub postfixes: Vec<String>,
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            value: Default::default(),
            prefixes: Default::default(),
            postfixes: Default::default(),
        }
    }
}

impl Clone for Letter {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            prefixes: self.prefixes.clone(),
            postfixes: self.postfixes.clone(),
        }
    }
}

#[derive(Resource)]
pub struct LetterList {
    pub letters: Vec<Letter>,
}

fn load_letters(mut commands: Commands) {
    commands.insert_resource(LetterList {
        letters: vec![Letter {
            value: "b".into(),
            prefixes: vec![
                "m".into(),
                "a".into(),
                "e".into(),
                "i".into(),
                "o".into(),
                "u".into(),
            ],
            postfixes: vec!["a".into(), "e".into(), "i".into(), "o".into(), "u".into()],
        }],
    });
}

fn spawn_letter(
    asset_server: Res<AssetServer>,
    letter_list: Res<LetterList>,
    mut commands: Commands,
) {
    let letter = letter_list.letters[0].clone();
    let texture = asset_server.load("images/b.png");

    commands.spawn((
        Name::new("Letter"),
        letter,
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_scale(Vec2::splat(2.0).extend(-1.0)),
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
