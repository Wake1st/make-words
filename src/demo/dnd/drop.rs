use bevy::prelude::*;

use crate::{
    demo::letters::word::{AddLettersToWord, RemoveWord, Word},
    AppSet,
};

use super::{cursor::store_cursor_position, drag::Dragging};

pub const DROP_ZONE_SIZE: Vec2 = Vec2::splat(256.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        drop.in_set(AppSet::RecordInput)
            .after(store_cursor_position),
    );
}

#[derive(Component, Debug)]
pub struct DropZone {}

fn drop(
    buttons: Res<ButtonInput<MouseButton>>,
    dragging: Query<(Entity, &Transform, &Word), With<Dragging>>,
    mut commands: Commands,
    drop_zones: Query<(Entity, &Transform, &Word)>,
    mut add_letter_event: EventWriter<AddLettersToWord>,
    mut remove_word_event: EventWriter<RemoveWord>,
) {
    //  Only end on mouse up
    if buttons.just_released(MouseButton::Left) {
        //  Only run if dragging exist
        for (dragging_entity, dragging_transform, dragging_word) in dragging.iter() {
            //  ensure letter is no longer "dragging"
            commands.entity(dragging_entity).remove::<Dragging>();

            //	check for drop zone
            for (drop_zone_entity, drop_zone_transform, drop_zone_word) in drop_zones.iter() {
                //  calculate adjustments for both words
                let drop_zone_adjustment = (drop_zone_word.letters.len() + 1) as f32 * 128.0;
                let dragging_adjustment = (dragging_word.letters.len() - 1) as f32 * 128.0;

                //	check if dragging is in the left drop zone
                let drop_zone_left_origin =
                    drop_zone_transform.translation.xy() - Vec2::new(drop_zone_adjustment, 0.0);
                let dragging_right_origin =
                    dragging_transform.translation.xy() + Vec2::new(dragging_adjustment, 0.0);
                let drop_zone_left = Rect::from_center_size(drop_zone_left_origin, DROP_ZONE_SIZE);

                if drop_zone_left.contains(dragging_right_origin) {
                    //  attach to dropped letter
                    add_letter_event.send(AddLettersToWord {
                        word: drop_zone_entity,
                        letters: dragging_word.letters.clone(),
                        links: dragging_word.links.clone(),
                        left_side: true,
                    });
                    //  remove old word
                    remove_word_event.send(RemoveWord {
                        word: dragging_entity,
                    });
                }

                //	check if dragging is in the right drop zone
                let drop_zone_right_origin =
                    drop_zone_transform.translation.xy() + Vec2::new(drop_zone_adjustment, 0.0);
                let dragging_left_origin =
                    dragging_transform.translation.xy() - Vec2::new(dragging_adjustment, 0.0);
                let drop_zone_right =
                    Rect::from_center_size(drop_zone_right_origin, DROP_ZONE_SIZE);

                if drop_zone_right.contains(dragging_left_origin) {
                    //  attach to dropped letter
                    add_letter_event.send(AddLettersToWord {
                        word: drop_zone_entity,
                        letters: dragging_word.letters.clone(),
                        links: dragging_word.links.clone(),
                        left_side: false,
                    });
                    //  remove old word
                    remove_word_event.send(RemoveWord {
                        word: dragging_entity,
                    });
                }
            }
        }
    }
}
