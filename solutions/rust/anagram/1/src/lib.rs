use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let word_lower = word.to_lowercase();
    let sorted_word = sort_chars(&word_lower);

    let mut result = HashSet::new();
    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        if candidate_lower == word_lower {
            continue;
        }

        if sort_chars(&candidate_lower) == sorted_word {
            result.insert(candidate);
        }
    }

    result
}

fn sort_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars
}