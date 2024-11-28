use bevy::{prelude::*, window::WindowResized};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(WindowSize {
        width: 900.0,
        height: 900.,
    });

    app.add_systems(Update, resize_notificator.in_set(AppSet::RecordInput));
}

#[derive(Resource)]
pub struct WindowSize {
    width: f32,
    height: f32,
}

fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut window_size: ResMut<WindowSize>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.read(&resize_event) {
        window_size.width = e.width;
        window_size.height = e.height;
    }
}
