use std::collections::HashSet;

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    return chars.into_iter().collect();
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut correct_anagrams = HashSet::with_capacity(possible_anagrams.len());

    for anagram in possible_anagrams.iter() {
        if sort_word(anagram) == sort_word(word) && word.to_lowercase() != anagram.to_lowercase() {
            correct_anagrams.insert(*anagram);
        }
    }

    return correct_anagrams;
}

