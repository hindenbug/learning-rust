use std::ascii::AsciiExt;

pub fn score(word: &str) -> u32 {
    word.chars()
        .map(|c| letter_score(c.to_ascii_lowercase())).sum()
}

pub fn letter_score(c: char) -> u32 {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0,
    }
}
