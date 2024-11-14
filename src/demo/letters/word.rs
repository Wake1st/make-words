use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    demo::{
        dnd::{drag::Draggable, drop::DropZone},
        letters::letter_links::{spawn_letter_link, SpawnLink},
        movement::ScreenWrap,
    },
    screens::Screen,
    AppSet,
};

use super::{
    letter::Letter,
    letter_links::{LetterLink, RemoveLetterLink},
};

const SHIFT_DISTANCE: f32 = 100.0;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<CreateNewWord>();
    app.add_event::<AddLettersToWord>();
    app.add_event::<RemoveLettersFromWord>();
    app.add_event::<RemoveWord>();
    app.add_event::<ShiftWord>();

    app.add_systems(
        Update,
        (
            create_new_word,
            add_letters_to_word,
            remove_letters_from_word,
            shift_word,
            (move_word_components, draw_drop_zones),
        )
            .chain()
            .in_set(AppSet::Update),
    )
    .add_systems(Update, remove_word.in_set(AppSet::Despawn));
}

#[derive(Component, Default)]
pub struct Word {
    pub letters: Vec<Entity>,
    pub links: Vec<Entity>,
}

#[derive(Event)]
pub struct CreateNewWord {
    pub letters: Vec<Entity>,
    pub links: Vec<Entity>,
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
                letters: event.letters.clone(),
                links: event.links.clone(),
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
pub struct AddLettersToWord {
    pub word: Entity,
    pub letters: Vec<Entity>,
    pub links: Vec<Entity>,
    pub left_side: bool,
}

fn add_letters_to_word(
    mut add_letters_event: EventReader<AddLettersToWord>,
    mut words: Query<(&Transform, &mut Word, &mut Draggable)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in add_letters_event.read() {
        info!(
            "adding letters {:?} to word {:?}",
            event.letters, event.word
        );
        if let Ok((transform, mut word, mut draggable)) = words.get_mut(event.word) {
            //  get the leftmost position of the word
            let texture = asset_server.load("images/link.png");
            let word_half_translation = Vec3::new(draggable.size.x / 2.0, 0.0, 0.0);

            //  add letters and links to the list
            if event.left_side {
                word.letters = [event.letters.clone(), word.letters.clone()].concat();
                word.links = [
                    event.links.clone(),
                    vec![spawn_letter_link(
                        SpawnLink {
                            image: texture.clone(),
                            position: transform.translation - word_half_translation,
                        },
                        &mut commands,
                    )],
                    word.links.clone(),
                ]
                .concat();
            } else {
                word.letters = [word.letters.clone(), event.letters.clone()].concat();
                word.links = [
                    word.links.clone(),
                    vec![spawn_letter_link(
                        SpawnLink {
                            image: texture.clone(),
                            position: transform.translation + word_half_translation,
                        },
                        &mut commands,
                    )],
                    event.links.clone(),
                ]
                .concat();
            }

            //  expend the draggable size
            draggable.size = Vec2::new(word.letters.len() as f32 * 256.0, 256.0);
        }
    }
}

#[derive(Event)]
pub struct RemoveLettersFromWord {
    pub word: Entity,
    pub link_index: usize,
    pub position: Vec2,
}

/// Removes the letters from the `(letter_index, len]`;
/// the remaining letters in the word will range from `[0, letter_index)`.
fn remove_letters_from_word(
    mut remove_letters_event: EventReader<RemoveLettersFromWord>,
    mut words: Query<(&mut Word, &mut Draggable)>,
    mut remove_link_event: EventWriter<RemoveLetterLink>,
    mut create_new_word: EventWriter<CreateNewWord>,
    mut shift_word_event: EventWriter<ShiftWord>,
) {
    for event in remove_letters_event.read() {
        if let Ok((mut word, mut draggable)) = words.get_mut(event.word) {
            //  remove the letters and create a new word for them
            let links = if event.link_index == word.links.len() - 1 {
                let split = word.links.split_off(0);
                remove_link_event.send(RemoveLetterLink { link: split[0] });

                Vec::new()
            } else {
                let mut split = word.links.split_off(event.link_index);
                let new_links = split.split_off(if split.len() == 1 { 0 } else { 1 });
                info!(
                    "split len(): {:?}\tword links len(): {:?}\tnew link len(): {:?}",
                    split.len(),
                    new_links.len(),
                    word.links.len(),
                );

                remove_link_event.send(RemoveLetterLink { link: split[0] });

                new_links
            };

            let letters = word.letters.split_off(event.link_index + 1);
            info!(
                "letters - \tbroke-off: {:?}\tremaining: {:?}",
                letters, word.letters
            );

            create_new_word.send(CreateNewWord {
                letters: letters.clone(),
                links,
                position: event.position
                    + Vec2::new(
                        letters.len() as f32 * 128.0 + SHIFT_DISTANCE,
                        SHIFT_DISTANCE,
                    ),
            });

            //  expend the draggable size
            draggable.size = Vec2::new(word.letters.len() as f32 * 256.0, 256.0);

            //  shift the old word away
            shift_word_event.send(ShiftWord {
                word: event.word,
                leftward: true,
            });
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

fn move_word_components(
    words: Query<(&Transform, &Word), (Without<Letter>, Without<LetterLink>)>,
    mut moving_set: ParamSet<(
        Query<&mut Transform, With<Letter>>,
        Query<&mut Transform, With<LetterLink>>,
    )>,
) {
    for (word_transform, word) in words.iter() {
        let word_half_length = word.letters.len() as f32 * 128.0;

        for (index, &letter) in word.letters.iter().enumerate() {
            if let Ok(mut letter_transform) = moving_set.p0().get_mut(letter) {
                let x =
                    word_transform.translation.x - word_half_length + 256.0 * (0.5 + index as f32);
                letter_transform.translation = Vec3::new(x, word_transform.translation.y, 0.0);
            }
        }

        for (index, &link) in word.links.iter().enumerate() {
            if let Ok(mut link_transform) = moving_set.p1().get_mut(link) {
                let x =
                    word_transform.translation.x - word_half_length + 256.0 * (1.0 + index as f32);
                link_transform.translation = Vec3::new(x, word_transform.translation.y, 0.0);
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

#[derive(Event)]
pub struct ShiftWord {
    pub word: Entity,
    pub leftward: bool,
}

fn shift_word(
    mut shift_event: EventReader<ShiftWord>,
    mut words: Query<&mut Transform, With<Word>>,
) {
    for event in shift_event.read() {
        if let Ok(mut transform) = words.get_mut(event.word) {
            let direction = if event.leftward { -1.0 } else { 1.0 };
            transform.translation +=
                Vec3::new(direction * SHIFT_DISTANCE, direction * SHIFT_DISTANCE, 0.0);
        }
    }
}
