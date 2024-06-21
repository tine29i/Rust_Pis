// modify_letter.rs

pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let letter_lower = letter.to_lowercase().next().unwrap();
    let letter_upper = letter.to_uppercase().next().unwrap();
    s.chars()
        .filter(|&c| c != letter_lower && c != letter_upper)
        .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let letter_lower = letter.to_lowercase().next().unwrap();
    let letter_upper = letter.to_uppercase().next().unwrap();
    s.chars()
        .map(|c| {
            if c == letter_lower {
                letter_upper
            } else if c == letter_upper {
                letter_lower
            } else {
                c
            }
        })
        .collect()
}
