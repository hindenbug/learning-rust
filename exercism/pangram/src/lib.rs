use std::ascii::AsciiExt;

pub fn is_pangram(text: &str) -> bool {

    let mut chars = text.to_lowercase().chars().filter(|&x| is_valid_char(x)).collect::<Vec<_>>();

    chars.sort();
    chars.dedup();

    match chars.len() {
        26 => true,
        _ => false
    }

}

pub fn is_valid_char(c: char) -> bool {
    c.is_ascii() && !c.is_whitespace() && c.is_alphabetic()
}
