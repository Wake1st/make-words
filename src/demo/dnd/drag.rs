use bevy::prelude::*;

use crate::{demo::letters::sounds::PlayWordSounds, AppSet};

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
pub struct Dragging {
    offset: Vec2,
}

fn start_drag(
    dragging: Query<&Dragging>,
    draggables: Query<(Entity, &Transform, &Draggable)>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut play_word_sounds: EventWriter<PlayWordSounds>,
) {
    //  Only start on mouse down
    if buttons.just_pressed(MouseButton::Left) {
        //  Check any dragging exist and return early if true
        if dragging.iter().next().is_some() {
            return;
        }

        //  Check for possible collision
        for (entity, transform, draggable) in draggables.iter() {
            let rect = Rect::from_center_size(transform.translation.xy(), draggable.size);

            if rect.contains(cursor_position.0) {
                //  add dragging component to the word
                commands.entity(entity).insert(Dragging {
                    offset: transform.translation.xy() - cursor_position.0,
                });

                //  play the sounds of the word
                play_word_sounds.send(PlayWordSounds { word: entity });
            }
        }
    }
}

fn drag(mut draggings: Query<(&mut Transform, &Dragging)>, cursor_position: Res<CursorPosition>) {
    for (mut transform, dragging) in draggings.iter_mut() {
        transform.translation = (cursor_position.0 + dragging.offset).extend(0.0);
    }
}
