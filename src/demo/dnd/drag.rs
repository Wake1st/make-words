use bevy::prelude::*;

use crate::AppSet;

use super::cursor::{store_cursor_position, CursorPosition};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            start_drag
                .in_set(AppSet::RecordInput)
                .after(store_cursor_position),
            drag.in_set(AppSet::Update),
        ),
    );
}

#[derive(Component, Debug)]
pub struct Draggable {
    pub size: Vec2,
}

#[derive(Component, Debug)]
pub struct Dragging {}

fn start_drag(
    dragging: Query<&Dragging>,
    draggables: Query<(Entity, &Transform, &Draggable)>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    //  Only start on mouse down
    if buttons.just_pressed(MouseButton::Left) {
        //  Check any dragging exist and return early if true
        for _ in dragging.iter() {
            return;
        }

        //  Check for possible collision
        for (entity, transform, draggable) in draggables.iter() {
            let rect = Rect::from_center_size(transform.translation.xy(), draggable.size);

            if rect.contains(cursor_position.0) {
                commands.entity(entity).insert(Dragging {});
            }
        }
    }
}

fn drag(mut dragging: Query<&mut Transform, With<Dragging>>, cursor_position: Res<CursorPosition>) {
    for mut transform in dragging.iter_mut() {
        transform.translation += cursor_position.0.extend(0.0);
    }
}
