use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CursorPosition(Vec2::ZERO));

    app.add_systems(Update, store_cursor_position.in_set(AppSet::RecordInput));
}

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

pub fn store_cursor_position(
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_position.0 = pos;
    }
}
