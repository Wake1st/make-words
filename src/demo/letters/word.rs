use bevy::prelude::*;

use crate::{
    demo::{
        dnd::{drag::Draggable, drop::DropZone},
        movement::ScreenWrap,
    },
    screens::Screen,
    AppSet,
};

use super::letter::Letter;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<CreateNewWord>();
    app.add_event::<AddLetterToWord>();
    app.add_event::<RemoveWord>();

    app.add_systems(
        Update,
        (create_new_word, add_letter_to_word)
            .chain()
            .in_set(AppSet::Update),
    )
    .add_systems(Update, remove_word.in_set(AppSet::Despawn));
}

#[derive(Component)]
pub struct Word {
    pub letters: Vec<Letter>,
}

impl Default for Word {
    fn default() -> Self {
        Self {
            letters: Vec::new(),
        }
    }
}

#[derive(Event)]
pub struct CreateNewWord {
    pub letter: Letter,
}

fn create_new_word(mut create_event: EventReader<CreateNewWord>, mut commands: Commands) {
    for event in create_event.read() {
        commands.spawn((
            Name::new("Word"),
            Word {
                letters: vec![event.letter.clone()],
            },
            Draggable {
                size: Vec2::splat(256.0),
            },
            DropZone {
                size: Vec2::splat(256.0),
            },
            ScreenWrap,
            StateScoped(Screen::Gameplay),
        ));
    }
}

#[derive(Event)]
pub struct AddLetterToWord {
    pub word_entity: Entity,
    pub letter: Letter,
    pub left_side: bool,
}

fn add_letter_to_word(
    mut add_letter_event: EventReader<AddLetterToWord>,
    mut words: Query<(&mut Word, &mut Draggable)>,
) {
    for event in add_letter_event.read() {
        if let Ok((mut word, mut draggable)) = words.get_mut(event.word_entity) {
            //  add letter to the list
            if event.left_side {
                word.letters.insert(0, event.letter.clone());
            } else {
                word.letters.push(event.letter.clone());
            }

            //  expend the draggable size
            draggable.size = Vec2::new(word.letters.len() as f32 * 256.0, 256.0);
        }
    }
}

#[derive(Event)]
pub struct RemoveWord {
    pub word_entity: Entity,
}

fn remove_word(mut remove_letter_event: EventReader<RemoveWord>, mut commands: Commands) {
    for event in remove_letter_event.read() {
        commands.entity(event.word_entity).despawn_recursive();
    }
}
