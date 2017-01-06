pub fn primes_up_to(num: usize) -> Vec<usize> {

    let mut primes = vec![];
    let num_sqrt = (num as f64).sqrt().ceil() as usize;

    'outer: for i in 2..num+1 {
        for x in  2..(num_sqrt + 1) {
            if i % x == 0 && i != x { continue 'outer }
        }
        primes.push(i)
    }

    primes
}

/* This too looks good http://exercism.io/submissions/e5e7eddc6d124d6c91e9e6fd5b43cc06

pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut candidates: Vec<_> = (2..limit+1).rev().collect();
    while let Some(prime) = candidates.pop() {
        primes.push(prime);
        candidates.retain(|n| n % prime != 0);
    }
    primes
}

*/
