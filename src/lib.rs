use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = &word.to_lowercase();
    let word_char_freq = char_freq(word_lower);
    possible_anagrams
        .into_iter()
        .filter(|possible| {
            let possible_lower = &possible.to_lowercase();
            word.len() == possible.len()
                && word_lower != possible_lower
                && word_char_freq == char_freq(possible_lower)
        })
        .map(|w| *w)
        .collect()
}

fn char_freq(w: &str) -> HashMap<char, u64> {
    w.chars().fold(HashMap::new(), |mut char_freq, c| {
        let count = char_freq.entry(c).or_default();
        *count += 1;
        char_freq
    })
}
