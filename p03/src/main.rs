/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?
*/

fn main() {
    println!("{:?}", get_max_prime_factor(600851475143))
}

fn get_max_prime_factor(num: u64) -> u64 {
    let primes = get_prime_factors(num);
    *primes.iter().to_owned().max().to_owned().unwrap()
}

fn get_prime_factors(num: u64) -> Vec<u64> {
    let s = (num as f64).sqrt();
    let mut r: Vec<u64> = Vec::new();
    for i in 2..=(s.floor() as u64) {
        if num % i == 0 {
            let mut prime = true;
            for p in &r {
                if i % p == 0 {
                    prime = false;
                }
            }
            if prime {
                r.push(i);
            }
        }
    }
    r
}

#[test]
fn test_get_prime_factors() {
    assert_eq!(get_prime_factors(13195), [5, 7, 13, 29]);
    assert_eq!(get_max_prime_factor(13195), 29);
}

#[test]
fn test_case() {
    assert_eq!(get_prime_factors(600851475143), [71, 839, 1471, 6857]);
    assert_eq!(get_max_prime_factor(600851475143), 6857);
}
