use bevy::prelude::*;

use crate::demo::drawer::instructions::IterateInstruction;

use super::{
    censor::allow_word,
    letter::Letter,
    word::{AddLettersToWord, RemoveWord, Word},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<AddToWordCheck>();

    app.add_systems(Update, add_to_word_check);
}

#[derive(Event)]
pub struct AddToWordCheck {
    pub appending_word: Entity,
    pub adding_letters: Vec<Entity>,
    pub adding_links: Vec<Entity>,
    pub adding_left: bool,
    pub removing_word: Entity,
}

fn add_to_word_check(
    mut add_to_word_event: EventReader<AddToWordCheck>,
    words: Query<&Word>,
    letter_query: Query<&Letter>,
    mut add_letter_event: EventWriter<AddLettersToWord>,
    mut remove_word_event: EventWriter<RemoveWord>,
    mut iterate_instruction: EventWriter<IterateInstruction>,
) {
    //  check if the word is banned
    for event in add_to_word_event.read() {
        //	get the appended word
        if let Ok(word) = words.get(event.appending_word) {
            //	organize the entities
            let mut letter_entities = Vec::new();
            if event.adding_left {
                letter_entities.append(&mut event.adding_letters.clone());
                letter_entities.append(&mut word.letters.clone());
            } else {
                letter_entities.append(&mut word.letters.clone());
                letter_entities.append(&mut event.adding_letters.clone());
            }

            //	assemble the letters
            let mut new_letters = Vec::new();
            for &entity in letter_entities.iter() {
                if let Ok(letter) = letter_query.get(entity) {
                    new_letters.push(letter.clone());
                }
            }

            if allow_word(new_letters.clone()) {
                //  attach to dropped letter
                add_letter_event.send(AddLettersToWord {
                    word: event.appending_word,
                    letters: event.adding_letters.clone(),
                    links: event.adding_links.clone(),
                    left_side: event.adding_left,
                });

                //  remove old word
                remove_word_event.send(RemoveWord {
                    word: event.removing_word,
                    remove_parts: false,
                });

                //  update instructions
                iterate_instruction.send(IterateInstruction { index: 2 });
            }
        }
    }
}
