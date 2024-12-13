use bevy::prelude::*;

use crate::demo::drawer::instructions::IterateInstruction;

use super::{
    letter::Letter,
    word::{AddLettersToWord, RemoveWord, Word},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<AddToWordCheck>();

    app.add_systems(
        Update,
        (
            add_to_word_check,
            remove_from_word_check,
            create__new_word_check,
        ),
    );
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
    letters: Query<&Letter>,
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
                if let Ok(letter) = letters.get(entity) {
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

fn remove_from_word_check() {}

fn create__new_word_check() {}

pub fn allow_word(letters: Vec<Letter>) -> bool {
    let banned = vec![
        "a", // 0
        "c", // 1
        "e", // 2
        "f", // 3
        "g", // 4
        "i", // 5
        "k", // 6
        "n", // 7
        "o", // 8
        "r", // 9
        "t", // 10
        "u", // 11
    ];

    for (index, letter) in letters.iter().enumerate() {
        //	first check
        if letter.value == banned[7] {
            let mut flag_check = true;

            flag_check = flag_check
                || (index + 1 < letters.len() - 1 && letters[index + 1].value == banned[5]);

            flag_check = flag_check
                || (index + 2 < letters.len() - 1 && letters[index + 2].value == banned[4]);

            flag_check = flag_check
                || (index + 3 < letters.len() - 1 && letters[index + 3].value == banned[4]);

            flag_check = flag_check
                || (index + 4 < letters.len() - 1 && letters[index + 4].value == banned[2]);

            flag_check = flag_check
                || (index + 5 < letters.len() - 1 && letters[index + 5].value == banned[9]);

            if flag_check {
                return false;
            }
        }

        //	second check
        if letter.value == banned[3] {
            let mut flag_check = true;

            flag_check = flag_check
                || (index + 1 < letters.len() - 1 && letters[index + 1].value == banned[0]);

            flag_check = flag_check
                || (index + 2 < letters.len() - 1 && letters[index + 2].value == banned[4]);

            flag_check = flag_check
                || (index + 3 < letters.len() - 1 && letters[index + 3].value == banned[4]);

            flag_check = flag_check
                || (index + 4 < letters.len() - 1 && letters[index + 4].value == banned[8]);

            flag_check = flag_check
                || (index + 5 < letters.len() - 1 && letters[index + 5].value == banned[10]);

            if flag_check {
                return false;
            }
        }

        //	third check
        if letter.value == banned[4] {
            let mut flag_check = true;

            flag_check = flag_check
                || (index + 1 < letters.len() - 1 && letters[index + 1].value == banned[8]);

            flag_check = flag_check
                || (index + 2 < letters.len() - 1 && letters[index + 2].value == banned[8]);

            flag_check = flag_check
                || (index + 3 < letters.len() - 1 && letters[index + 3].value == banned[6]);

            if flag_check {
                return false;
            }
        }

        //	forth check
        if letter.value == banned[1] {
            let mut flag_check = true;

            flag_check = flag_check
                || (index + 1 < letters.len() - 1 && letters[index + 1].value == banned[11]);

            flag_check = flag_check
                || (index + 2 < letters.len() - 1 && letters[index + 2].value == banned[7]);

            flag_check = flag_check
                || (index + 3 < letters.len() - 1 && letters[index + 3].value == banned[10]);

            if flag_check {
                return false;
            }
        }
    }

    //	pass if nothing was flagged
    return true;
}
