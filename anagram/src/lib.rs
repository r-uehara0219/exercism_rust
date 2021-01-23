// use std::collections::HashSet;

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
//     unimplemented!(
//         "For the '{}' word find anagrams among the following words: {:?}",
//         word,
//         possible_anagrams
//     );
// }


use std::collections::HashSet;

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    return chars.into_iter().collect();
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut correct_anagrams = HashSet::with_capacity(6);
    let sorted_word: String = sort_word(word);

    for anagram in possible_anagrams.iter() {
        if sort_word(anagram) == sorted_word && anagram != &word {
            correct_anagrams.insert(*anagram);
        }
    }

    return correct_anagrams;
}

