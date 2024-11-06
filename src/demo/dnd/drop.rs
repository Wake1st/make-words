use bevy::prelude::*;

use crate::AppSet;

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

#[derive(Component, Debug)]
pub struct DropZone {
    pub size: Vec2,
    pub left_side: Entity,
    pub right_side: Entity,
}

impl Default for DropZone {
    fn default() -> Self {
        Self {
            size: Default::default(),
            left_side: Entity::PLACEHOLDER,
            right_side: Entity::PLACEHOLDER,
        }
    }
}

fn drop(
    buttons: Res<ButtonInput<MouseButton>>,
    dragging: Query<Entity, With<Dragging>>,
    mut drop_zones: Query<(&Transform, &mut DropZone)>,
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
) {
    //  Only end on mouse up
    if buttons.just_released(MouseButton::Left) {
        //  Only run if dragging exist
        for entity in dragging.iter() {
            //  ensure letter is no longer "dragging"
            commands.entity(entity).remove::<Dragging>();

            //	check for drop zone
            for (transform, mut drop_zone) in drop_zones.iter_mut() {
                //	remove if mouse is in the left or right drop zone
                let left_translation = transform.translation.xy() + Vec2::new(128.0, 0.0);
                let left_rect = Rect::from_center_size(left_translation, drop_zone.size);
                if left_rect.contains(cursor_position.0) {
                    //  attach to dropped letter
                    drop_zone.left_side = entity;
                }

                let right_translation = transform.translation.xy() - Vec2::new(128.0, 0.0);
                let right_rect = Rect::from_center_size(right_translation, drop_zone.size);
                if right_rect.contains(cursor_position.0) {
                    //  attach to dropped letter
                    drop_zone.right_side = entity;
                }
            }
        }
    }
}
