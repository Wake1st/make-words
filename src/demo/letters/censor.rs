use super::letter::Letter;

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

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1].value == banned[5]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2].value == banned[4]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3].value == banned[4]);

            flag_check =
                flag_check && (index + 4 < letters.len() && letters[index + 4].value == banned[2]);

            flag_check =
                flag_check && (index + 5 < letters.len() && letters[index + 5].value == banned[9]);

            if flag_check {
                return false;
            }
        }

        //	second check
        if letter.value == banned[3] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1].value == banned[0]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2].value == banned[4]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3].value == banned[4]);

            flag_check =
                flag_check && (index + 4 < letters.len() && letters[index + 4].value == banned[8]);

            flag_check =
                flag_check && (index + 5 < letters.len() && letters[index + 5].value == banned[10]);

            if flag_check {
                return false;
            }
        }

        //	third check
        if letter.value == banned[4] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1].value == banned[8]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2].value == banned[8]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3].value == banned[6]);

            if flag_check {
                return false;
            }
        }

        //	forth check
        if letter.value == banned[1] {
            let mut flag_check = true;

            flag_check =
                flag_check && (index + 1 < letters.len() && letters[index + 1].value == banned[11]);

            flag_check =
                flag_check && (index + 2 < letters.len() && letters[index + 2].value == banned[7]);

            flag_check =
                flag_check && (index + 3 < letters.len() && letters[index + 3].value == banned[10]);

            if flag_check {
                return false;
            }
        }
    }

    //	pass if nothing was flagged
    return true;
}
