use std::collections::HashMap;

pub fn count(c: char, s: &str) -> usize {

    let counter = s.chars().filter(|&ch| ch == c).collect::<Vec<_>>();
    counter.len()

}


pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {

    /*
    This solution only to learn more about HashMap ``entry`` ``or_insert``
    Increment HashMap value by iterating
    let mut nucleotide_freq: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)].iter().cloned().collect();

    for c in s.chars() {
        let counter = nucleotide_freq.entry(c).or_insert(0);
        *counter += 1;
    };

    nucleotide_freq
    */


    [('A', count('A', s)), ('C', count('C', s)), ('G', count('G', s)), ('T', count('T', s))]
        .iter().cloned().collect()
}
