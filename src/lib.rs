use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort();
    HashSet::from_iter(possible_anagrams.iter().filter(|&anagram| {
        let anagram = anagram.to_lowercase();
        let mut anagram_sorted: Vec<char> = anagram.chars().collect();
        anagram_sorted.sort();
        anagram_sorted == word_sorted 
            && anagram != word       
    }).cloned())
}
