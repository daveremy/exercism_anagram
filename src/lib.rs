use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_num = to_number(word);
    possible_anagrams
        .into_iter()
        .filter(|w| word_num == to_number(w) && word != **w)
        .map(|w| *w)
        .collect()
}

/// Convert the word to a unique number:
fn to_number(word: &str) -> u64 {
    word.chars().fold(0, |acc, c| {
        acc + c
            .to_string()
            .as_bytes()
            .iter()
            // create a unique number for each character by multiplying each
            // byte by an increasing magnitude in order to assure ordering of
            // bytes matter
            .fold((0, 1), |acc, b| (acc.1 * *b as u64, acc.1 * 10))
            .0
    })
}
