pub fn lsp(number: &str, size: usize) -> Result<u32, &str> {

    if number.len() < size {
        return Err("Invalid");
    }

    if size == 0 {
        return Ok(1);
    }

    let mut digits = vec![];

    for c in number.chars() {
        match c.to_digit(10) {
            Some(d) => digits.push(d),
            _ => return Err("Invalid character."),
        }
    }

    Ok(digits.windows(size)
       .map(|s| s.iter().product())
       .max()
       .unwrap())
}
