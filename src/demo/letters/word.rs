use bevy::{color::palettes::css::WHITE, prelude::*};

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
        (
            create_new_word,
            add_letter_to_word,
            (move_letters_in_word, draw_drop_zones),
        )
            .chain()
            .in_set(AppSet::Update),
    )
    .add_systems(Update, remove_word.in_set(AppSet::Despawn));
}

#[derive(Component)]
pub struct Word {
    pub letters: Vec<Entity>,
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
    pub letter: Entity,
    pub position: Vec2,
}

fn create_new_word(mut create_event: EventReader<CreateNewWord>, mut commands: Commands) {
    for event in create_event.read() {
        commands.spawn((
            Name::new("Word"),
            TransformBundle {
                local: Transform::from_translation(event.position.extend(0.0)),
                ..default()
            },
            Word {
                letters: vec![event.letter],
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
    pub word: Entity,
    pub letters: Vec<Entity>,
    pub left_side: bool,
}

fn add_letter_to_word(
    mut add_letter_event: EventReader<AddLetterToWord>,
    mut words: Query<(&mut Word, &mut Draggable)>,
) {
    for event in add_letter_event.read() {
        info!(
            "adding letters {:?} to word {:?}",
            event.letters, event.word
        );
        if let Ok((mut word, mut draggable)) = words.get_mut(event.word) {
            //  add letter to the list
            if event.left_side {
                word.letters = [event.letters.clone(), word.letters.clone()].concat();
            } else {
                word.letters = [word.letters.clone(), event.letters.clone()].concat();
            }

            //  expend the draggable size
            draggable.size = Vec2::new(word.letters.len() as f32 * 256.0, 256.0);
        }
    }
}

#[derive(Event)]
pub struct RemoveWord {
    pub word: Entity,
}

fn remove_word(mut remove_letter_event: EventReader<RemoveWord>, mut commands: Commands) {
    for event in remove_letter_event.read() {
        info!("removing {:?}", event.word);
        commands.entity(event.word).despawn_recursive();
    }
}

fn move_letters_in_word(
    words: Query<(&Transform, &Word)>,
    mut letters: Query<&mut Transform, (With<Letter>, Without<Word>)>,
) {
    for (word_transform, word) in words.iter() {
        let word_half_length = word.letters.len() as f32 * 128.0;

        for (index, &letter) in word.letters.iter().enumerate() {
            if let Ok(mut letter_transform) = letters.get_mut(letter) {
                let x =
                    word_transform.translation.x - word_half_length + 256.0 * (0.5 + index as f32);
                letter_transform.translation = Vec3::new(x, word_transform.translation.y, 0.0);
            }
        }
    }
}

fn draw_drop_zones(words: Query<(&Transform, &Word)>, mut gizmos: Gizmos) {
    for (transform, word) in words.iter() {
        let zone_adjustment = Vec2::new((word.letters.len() + 1) as f32 * 128.0, 0.0);

        //	remove if mouse is in the left drop zone
        let left_translation = transform.translation.xy() - zone_adjustment;
        gizmos.rect_2d(left_translation, 0., Vec2::splat(256.), WHITE);

        let right_translation = transform.translation.xy() + zone_adjustment;
        gizmos.rect_2d(right_translation, 0., Vec2::splat(256.), WHITE);
    }
}
