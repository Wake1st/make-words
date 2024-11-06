use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CursorPosition(Vec2::ZERO));

    app.add_systems(Update, store_cursor_position.in_set(AppSet::RecordInput));
}

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

pub fn store_cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let window = q_windows.single();

    // Games typically only have one window (the primary window)
    if let Some(position) = window.cursor_position() {
        //  move the "cursor" image
        cursor_position.0 = position;
    }
}
