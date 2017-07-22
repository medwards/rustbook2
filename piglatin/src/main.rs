use std::env;

fn main() {
    let words: Vec<_> = env::args().skip(1).map(|w| to_piglatin(&w)).collect();
    if words.len() == 0 {
        print_help();
        return;
    }
    for word in words.iter() {
        print!("{} ", word);
    }
    println!("");
}

fn to_piglatin(word: &String) -> String {
    let mut characters = word.chars();
    let mut part_2 = characters.next().unwrap();
    if is_vowel(&part_2) {
        format!("{}{}", word, "hay")
    } else {
        let mut part_1 = "";
        if part_2.is_uppercase() {
            part_2 = part_2.to_lowercase().next().unwrap(); // TODO: Should always be one character?
            part_1 = characters.as_str(); // TODO: Fix case when original word was capitalized
        } else {
            part_1 = characters.as_str();
        }
        // TODO: Fix period at end of word (ignore other punctuation issues)
        format!("{}{}{}", part_1, part_2, "ay")
    }
}

fn is_vowel(character: &char) -> bool {
    // TODO: Should "y" be considered a vowel?
    "aeiou".chars().any(|c| c == *character)
}

fn print_help() {
    println!("Usage: piglatin WORD [WORD [WORD]]");
    println!("At least one WORD is required");
}
