use bevy::prelude::*;

use crate::{demo::dnd::cursor::CursorPosition, AppSet};

use super::word::{RemoveLettersFromWord, Word};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (break_words.in_set(AppSet::Update),));
}

///	Links the letters together; when selected,
/// a word will break in two at the link.
#[derive(Component, Clone)]
pub struct LetterLink {
    word: Entity,
    index: usize,
    size: Vec2,
}

fn break_words(
    buttons: Res<ButtonInput<MouseButton>>,
    links: Query<(&Transform, &LetterLink)>,
    cursor_position: Res<CursorPosition>,
    mut remove_letters_event: EventWriter<RemoveLettersFromWord>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        for (link_transform, link) in links.iter() {
            //	ensure the cursor is hovered over the link
            let link_rect = Rect::from_center_size(link_transform.translation.xy(), link.size);
            if link_rect.contains(cursor_position.0) {
                //	break the word!
                remove_letters_event.send(RemoveLettersFromWord {
                    word: link.word,
                    letter_index: link.index + 1,
                    position: link_transform.translation.xy(),
                });
            }
        }
    }
}

#[derive(Event)]
pub struct ReIndexLetterLink {}

fn calculate_link_index(
    mut reindex_event: EventReader<ReIndexLetterLink>,
    mut words: Query<(Entity, &Word)>,
    mut links: Query<&mut LetterLink>,
) {
    for event in reindex_event.read() {
        for (word_entity, word) in words.iter() {
            for link in links.iter_mut() {
                if link.word == word_entity {}
            }
        }
    }
}
