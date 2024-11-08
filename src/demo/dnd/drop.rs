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
}

fn drop(
    dragging: Query<&Dragging>,
    drop_zones: Query<(Entity, &Transform, &DropZone)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
) {
    //  Only end on mouse up
    if buttons.just_released(MouseButton::Left) {
        //  Only run if dragging exist
        for _ in dragging.iter() {
            //	check for drop zone
            for (entity, transform, drop_zone) in drop_zones.iter() {
                //	remove if mouse is in the drop zone
                let rect = Rect::from_center_size(transform.translation.xy(), drop_zone.size);
                if rect.contains(cursor_position.0) {
                    commands.entity(entity).remove::<Dragging>();
                }
            }
        }
    }
}
