use bevy::prelude::*;

use crate::{
    demo::{
        drawer::instructions::IterateInstruction,
        letters::{
            censor::AddToWordCheck,
            trash::TrashCanDimentions,
            word::{RemoveWord, Word},
        },
    },
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
    mut remove_word_event: EventWriter<RemoveWord>,
    trash_can: Res<TrashCanDimentions>,
    mut iterate_instruction: EventWriter<IterateInstruction>,
    mut add_to_word_event: EventWriter<AddToWordCheck>,
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
                let dragging_xy = dragging_transform.translation.xy();

                //	check if dragging is in the left drop zone
                let drop_zone_left_origin =
                    drop_zone_transform.translation.xy() - Vec2::new(drop_zone_adjustment, 0.0);
                let dragging_right_origin = dragging_xy + Vec2::new(dragging_adjustment, 0.0);
                let drop_zone_left = Rect::from_center_size(drop_zone_left_origin, DROP_ZONE_SIZE);

                if drop_zone_left.contains(dragging_right_origin) {
                    //  check if the word is banned
                    add_to_word_event.send(AddToWordCheck {
                        appending_word: drop_zone_entity,
                        removing_word: dragging_entity,
                        adding_letters: dragging_word.letters.clone(),
                        adding_links: dragging_word.links.clone(),
                        adding_left: true,
                    });

                    //  exit to ensure this process happens ONCE
                    return;
                }

                //	check if dragging is in the right drop zone
                let drop_zone_right_origin =
                    drop_zone_transform.translation.xy() + Vec2::new(drop_zone_adjustment, 0.0);
                let dragging_left_origin = dragging_xy - Vec2::new(dragging_adjustment, 0.0);
                let drop_zone_right =
                    Rect::from_center_size(drop_zone_right_origin, DROP_ZONE_SIZE);

                if drop_zone_right.contains(dragging_left_origin) {
                    //  check if the word is banned
                    add_to_word_event.send(AddToWordCheck {
                        appending_word: drop_zone_entity,
                        removing_word: dragging_entity,
                        adding_letters: dragging_word.letters.clone(),
                        adding_links: dragging_word.links.clone(),
                        adding_left: false,
                    });

                    //  exit to ensure this process happens ONCE
                    return;
                }

                //  finally, check for trash dump
                let dragging_word_size =
                    Vec2::new(dragging_word.letters.len() as f32 * 256.0, 256.0);
                let dragging_rect = Rect::from_center_size(dragging_xy, dragging_word_size);
                if dragging_rect.contains(trash_can.rect.center()) {
                    //  remove the complete word
                    remove_word_event.send(RemoveWord {
                        word: dragging_entity,
                        remove_parts: true,
                    });

                    //  update instructions
                    iterate_instruction.send(IterateInstruction { index: 4 });
                }
            }
        }
    }
}
