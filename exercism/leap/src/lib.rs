pub fn is_leap_year(year: i32) -> bool {

    if divisible_by(year, 4) && (divisible_by(year, 400) || !divisible_by(year, 100)) {
        true
    } else {
        false
    }
}

fn divisible_by(a: i32, b:i32) -> bool {
    a % b == 0
}
