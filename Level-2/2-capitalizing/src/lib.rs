pub fn capitalize_first(input: &str) -> String {
    if let Some(first) = input.chars().next() {
        let mut capitalized = first.to_uppercase().to_string();
        capitalized.push_str(&input[1..]);
        capitalized
    } else {
        String::new()
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn change_case (input: &str) -> String{
    let mut result = String::new();
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            result.push_str(&c.to_uppercase().to_string());
        } else {
            result.push_str(&c.to_lowercase().to_string());
        }
    }
    result
}