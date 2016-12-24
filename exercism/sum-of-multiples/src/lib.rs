pub fn sum_of_multiples(n: i32, nset: &Vec<i32>) -> i32 {
    (0..n)
        .filter(|&x| is_multiple(x, nset))
        .sum()
}

pub fn is_multiple(x: i32, nset: &Vec<i32>) -> bool {
     nset.iter().any(|&i| x % i == 0)
}
