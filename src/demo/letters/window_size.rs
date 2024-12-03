use bevy::{prelude::*, window::WindowResized};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<WindowSizeUpdated>();

    app.insert_resource(WindowSize {
        size: Vec2::new(900.0, 900.0),
    });

    app.add_systems(Update, resize_notificator.in_set(AppSet::RecordInput));
}

#[derive(Resource)]
pub struct WindowSize {
    pub size: Vec2,
}

#[derive(Event)]
pub struct WindowSizeUpdated {
    pub size: Vec2,
}

fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut window_size: ResMut<WindowSize>,
    mut resize_notification: EventWriter<WindowSizeUpdated>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.read(&resize_event) {
        window_size.size.x = e.width;
        window_size.size.y = e.height;

        resize_notification.send(WindowSizeUpdated {
            size: window_size.size,
        });
    }
}
