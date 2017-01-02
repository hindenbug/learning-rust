use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {

    let mut result: HashMap<String, u32> = HashMap::new();

    let words = phrase.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty());

    for word in words {
        *result.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    result
}
