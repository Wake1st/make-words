pub fn allow_word(letters: Vec<String>) -> bool {
    let banned = ["a", "c", "e", "f", "g", "i", "k", "n", "o", "r", "t", "u"];

    for (index, letter) in letters.iter().enumerate() {
        //	first check
        if letter == banned[7] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1] == banned[5]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2] == banned[4]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3] == banned[4]);

            flag_check =
                flag_check && (index + 4 < letters.len() && letters[index + 4] == banned[2]);

            flag_check =
                flag_check && (index + 5 < letters.len() && letters[index + 5] == banned[9]);

            if flag_check {
                return false;
            }
        }

        //	second check
        if letter == banned[3] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1] == banned[0]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2] == banned[4]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3] == banned[4]);

            flag_check =
                flag_check && (index + 4 < letters.len() && letters[index + 4] == banned[8]);

            flag_check =
                flag_check && (index + 5 < letters.len() && letters[index + 5] == banned[10]);

            if flag_check {
                return false;
            }
        }

        //	third check
        if letter == banned[4] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1] == banned[8]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2] == banned[8]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3] == banned[6]);

            if flag_check {
                return false;
            }
        }

        //	forth check
        if letter == banned[1] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1] == banned[11]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2] == banned[7]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3] == banned[10]);

            if flag_check {
                return false;
            }
        }
    }

    //	pass if nothing was flagged
    true
}

#[test]
fn test_censor() {
    let banned = ["a", "c", "e", "f", "g", "i", "k", "n", "o", "r", "t", "u"];

    assert!(!allow_word(vec![
        banned[7].into(),
        banned[5].into(),
        banned[4].into(),
        banned[4].into(),
        banned[2].into(),
        banned[9].into()
    ]));
    assert!(!allow_word(vec![
        banned[3].into(),
        banned[0].into(),
        banned[4].into(),
        banned[4].into(),
        banned[8].into(),
        banned[10].into()
    ]));
    assert!(!allow_word(vec![
        banned[4].into(),
        banned[8].into(),
        banned[8].into(),
        banned[6].into()
    ]));
    assert!(!allow_word(vec![
        banned[1].into(),
        banned[11].into(),
        banned[7].into(),
        banned[10].into()
    ]));
}
