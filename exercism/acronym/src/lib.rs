pub fn abbreviate(phrase: &str) -> String {

    phrase.replace("-", " ")
        .split(|c: char| !c.is_alphanumeric() && c.is_whitespace())
        .flat_map(|word| split_word(word))
        .collect::<String>()
        .to_uppercase()
}

fn split_word(word: &str) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut s1: Vec<String> = Vec::new();

    for (i, c) in word.chars().enumerate() {
        if i == 0 || i != word.len() - 1 && c.is_uppercase() && chars[i-1].is_lowercase() && chars[i+1].is_lowercase() {
            s1.push(c.to_string());
        }
    }

    words.push(s1.iter().cloned().collect::<String>());
    words
}
