pub fn hamming_distance<'a>(strand1: &'a str, strand2: &'a str) -> Result<usize, &'a str> {

    if strand1.len() != strand2.len(){
        return  Err("Strands uncomparable")
    }else{
        Ok(strand1.chars()
           .zip(strand2.chars())
           .filter(|&(x, y)| x != y)
           .count())
    }

}
