use bevy::prelude::*;

use crate::{
    demo::{
        dnd::{drag::Draggable, drop::DropZone},
        letters::letter_links::{spawn_letter_link, SpawnLink},
    },
    screens::Screen,
    AppSet,
};

use super::{
    letter::{Letter, RemoveLetters},
    letter_links::{LetterLink, RemoveLetterLink, LETTER_LINK_LAYER},
    sounds::PlayWordSounds,
};

const SHIFT_DISTANCE: f32 = 100.0;
const DROP_ZONE_LAYER: f32 = -0.1;

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
            move_word_components,
        )
            .chain()
            .in_set(AppSet::Update),
    )
    .add_systems(Update, remove_word.in_set(AppSet::Despawn));
}

#[derive(Component)]
pub struct Word {
    pub letters: Vec<Entity>,
    pub links: Vec<Entity>,
    pub drop_zone_left: Entity,
    pub drop_zone_right: Entity,
}

#[derive(Event)]
pub struct CreateNewWord {
    pub letters: Vec<Entity>,
    pub links: Vec<Entity>,
    pub position: Vec2,
}

fn create_new_word(
    mut create_event: EventReader<CreateNewWord>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in create_event.read() {
        let texture = asset_server.load("images/blank.png");
        let mut transform = Transform::from_translation(event.position.extend(0.0));
        transform.translation.z += DROP_ZONE_LAYER;
        transform.scale *= 2.0;

        let left_drop_zone = create_drop_zone(&mut commands, texture.clone(), transform);
        let right_drop_zone = create_drop_zone(&mut commands, texture.clone(), transform);

        transform.scale *= 0.5;
        transform.translation.z -= DROP_ZONE_LAYER;
        commands.spawn((
            Name::new("Word"),
            TransformBundle {
                local: transform,
                ..default()
            },
            Word {
                letters: event.letters.clone(),
                links: event.links.clone(),
                drop_zone_left: left_drop_zone,
                drop_zone_right: right_drop_zone,
            },
            Draggable {
                size: Vec2::new(event.letters.len() as f32 * 256.0, 256.0),
            },
            StateScoped(Screen::Gameplay),
        ));
    }
}

fn create_drop_zone(
    commands: &mut Commands,
    texture: Handle<Image>,
    transform: Transform,
) -> Entity {
    commands
        .spawn((
            Name::new("DropZone"),
            SpriteBundle {
                texture,
                transform,
                ..Default::default()
            },
            DropZone {},
            StateScoped(Screen::Gameplay),
        ))
        .id()
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
    mut play_word_sounds: EventWriter<PlayWordSounds>,
) {
    for event in add_letters_event.read() {
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

            //  play the sound of the combined word
            play_word_sounds.send(PlayWordSounds { word: event.word });
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
                let split = word.links.split_off(event.link_index);
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
    pub remove_parts: bool,
}

fn remove_word(
    mut remove_letter_event: EventReader<RemoveWord>,
    words: Query<&Word>,
    mut commands: Commands,
    mut remove_letters: EventWriter<RemoveLetters>,
    mut remove_link: EventWriter<RemoveLetterLink>,
) {
    for event in remove_letter_event.read() {
        if let Ok(word) = words.get(event.word) {
            //  despawn the drop zones
            commands.entity(word.drop_zone_left).despawn_recursive();
            commands.entity(word.drop_zone_right).despawn_recursive();

            if event.remove_parts {
                //  despawn the letters
                remove_letters.send(RemoveLetters {
                    letters: word.letters.clone(),
                });

                //  despawn the links
                for &link in word.links.iter() {
                    remove_link.send(RemoveLetterLink { link });
                }
            }
        }

        //  despawn the word
        commands.entity(event.word).despawn_recursive();
    }
}

fn move_word_components(
    words: Query<(&Transform, &Word), (Without<Letter>, Without<LetterLink>, Without<DropZone>)>,
    mut moving_set: ParamSet<(
        Query<&mut Transform, With<Letter>>,
        Query<&mut Transform, With<LetterLink>>,
        Query<&mut Transform, With<DropZone>>,
    )>,
) {
    for (word_transform, word) in words.iter() {
        let word_half_length = word.letters.len() as f32 * 128.0;

        //  move letters
        for (index, &letter) in word.letters.iter().enumerate() {
            if let Ok(mut letter_transform) = moving_set.p0().get_mut(letter) {
                let x =
                    word_transform.translation.x - word_half_length + 256.0 * (0.5 + index as f32);
                letter_transform.translation = Vec3::new(x, word_transform.translation.y, 0.0);
            }
        }

        //  move links
        for (index, &link) in word.links.iter().enumerate() {
            if let Ok(mut link_transform) = moving_set.p1().get_mut(link) {
                let x =
                    word_transform.translation.x - word_half_length + 256.0 * (1.0 + index as f32);
                link_transform.translation =
                    Vec3::new(x, word_transform.translation.y, LETTER_LINK_LAYER);
            }
        }

        //  move drop zones
        if let Ok(mut link_transform) = moving_set.p2().get_mut(word.drop_zone_left) {
            let x = word_transform.translation.x - word_half_length - 128.0;
            link_transform.translation =
                Vec3::new(x, word_transform.translation.y, DROP_ZONE_LAYER);
        }
        if let Ok(mut link_transform) = moving_set.p2().get_mut(word.drop_zone_right) {
            let x = word_transform.translation.x + word_half_length + 128.0;
            link_transform.translation =
                Vec3::new(x, word_transform.translation.y, DROP_ZONE_LAYER);
        }
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
