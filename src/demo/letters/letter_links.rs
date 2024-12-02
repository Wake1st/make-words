use bevy::prelude::*;

use crate::{
    demo::{
        dnd::{cursor::CursorPosition, drag::Draggable},
        movement::ScreenWrap,
    },
    screens::Screen,
    AppSet,
};

use super::word::{RemoveLettersFromWord, Word};

const LINK_SIZE: Vec2 = Vec2::new(32.0, 256.0);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<RemoveLetterLink>();

    app.add_systems(Update, break_word.in_set(AppSet::Update))
        .add_systems(Update, despawn_link.in_set(AppSet::Despawn));
}

///    Links the letters together; when selected,
///    a word will break in two at the link.
#[derive(Component, Clone)]
pub struct LetterLink {
    size: Vec2,
}

pub struct SpawnLink {
    pub image: Handle<Image>,
    pub position: Vec3,
}

pub fn spawn_letter_link(spawn: SpawnLink, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("LetterLink"),
            SpriteBundle {
                texture: spawn.image.clone(),
                transform: Transform::from_translation(spawn.position),
                ..default()
            },
            LetterLink { size: LINK_SIZE },
            ScreenWrap,
            StateScoped(Screen::Gameplay),
        ))
        .id()
}

fn break_word(
    buttons: Res<ButtonInput<MouseButton>>,
    words: Query<(Entity, &Transform, &Draggable, &Word)>,
    links: Query<(&Transform, &LetterLink)>,
    cursor_position: Res<CursorPosition>,
    mut remove_letters_event: EventWriter<RemoveLettersFromWord>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        //	check if cursor is hovered over a word
        for (word_entity, word_transform, draggable, word) in words.iter() {
            let word_rect = Rect::from_center_size(word_transform.translation.xy(), draggable.size);
            if word_rect.contains(cursor_position.0) {
                // check if cursor is hovered over the word's links
                for (index, &link_entity) in word.links.iter().enumerate() {
                    if let Ok((link_transform, link)) = links.get(link_entity) {
                        let link_rect =
                            Rect::from_center_size(link_transform.translation.xy(), link.size);
                        if link_rect.contains(cursor_position.0) {
                            //  remove the letters from the right side
                            remove_letters_event.send(RemoveLettersFromWord {
                                word: word_entity,
                                link_index: index,
                                position: link_transform.translation.xy(),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct RemoveLetterLink {
    pub link: Entity,
}

fn despawn_link(mut remove_event: EventReader<RemoveLetterLink>, mut commands: Commands) {
    for event in remove_event.read() {
        commands.entity(event.link).despawn_recursive();
    }
}
