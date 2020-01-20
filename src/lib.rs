use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let sorted_chars = sort(&mut word.to_lowercase());
    possible_anagrams
        .into_iter()
        .filter(|w| {
            word.to_lowercase() != *w.to_lowercase() && sorted_chars == sort(&mut w.to_lowercase())
        })
        .map(|w| *w)
        .collect()
}

fn sort(w: &mut String) -> Vec<char> {
    let mut v: Vec<char> = w.chars().collect();
    v.sort();
    v
}
