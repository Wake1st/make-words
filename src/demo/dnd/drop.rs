use bevy::prelude::*;

use crate::{
    demo::letters::{
        letter::Letter,
        word::{AddLetterToWord, RemoveWord},
    },
    AppSet,
};

use super::{
    cursor::{store_cursor_position, CursorPosition},
    drag::Dragging,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        drop.in_set(AppSet::RecordInput)
            .after(store_cursor_position),
    );
}

//  TODO: Attach drop zone to words, not letters
#[derive(Component, Debug)]
pub struct DropZone {
    pub size: Vec2,
}

impl Default for DropZone {
    fn default() -> Self {
        Self {
            size: Default::default(),
        }
    }
}

fn drop(
    buttons: Res<ButtonInput<MouseButton>>,
    dragging: Query<(Entity, &Letter), With<Dragging>>,
    drop_zones: Query<(Entity, &Transform, &DropZone, &Letter)>,
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    mut add_letter_event: EventWriter<AddLetterToWord>,
    mut remove_word_event: EventWriter<RemoveWord>,
) {
    //  Only end on mouse up
    if buttons.just_released(MouseButton::Left) {
        //  Only run if dragging exist
        for (entity, dragging_letter) in dragging.iter() {
            //  ensure letter is no longer "dragging"
            commands.entity(entity).remove::<Dragging>();

            //	check for drop zone
            for (word_entity, transform, drop_zone, drop_zone_letter) in drop_zones.iter() {
                //	remove if mouse is in the left or right drop zone
                let left_translation = transform.translation.xy() + Vec2::new(128.0, 0.0);
                let left_rect = Rect::from_center_size(left_translation, drop_zone.size);
                if left_rect.contains(cursor_position.0) {
                    //  ensure the letter is matching
                    if drop_zone_letter.suffixes.contains(&dragging_letter.value)
                        && dragging_letter.prefixes.contains(&drop_zone_letter.value)
                    {
                        //  attach to dropped letter
                        add_letter_event.send(AddLetterToWord {
                            word_entity,
                            letter: dragging_letter.clone(),
                            left_side: true,
                        });
                        remove_word_event.send(RemoveWord {
                            word_entity: entity,
                        });
                        info!("droppen left yo!");
                    }
                }

                let right_translation = transform.translation.xy() - Vec2::new(128.0, 0.0);
                let right_rect = Rect::from_center_size(right_translation, drop_zone.size);
                if right_rect.contains(cursor_position.0) {
                    //  ensure the letter is matching
                    if drop_zone_letter.prefixes.contains(&dragging_letter.value)
                        && dragging_letter.suffixes.contains(&drop_zone_letter.value)
                    {
                        //  attach to dropped letter
                        add_letter_event.send(AddLetterToWord {
                            word_entity,
                            letter: dragging_letter.clone(),
                            left_side: false,
                        });
                        remove_word_event.send(RemoveWord {
                            word_entity: entity,
                        });
                        info!("droppen right yo!");
                    }
                }
            }
        }
    }
}
