/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

fn main() {
    println!("{}", get_prime_at(10));
}

fn get_prime_at(i: u64) -> u64 {
    13
};

fn get_primes(l: u64) {
    let mut primes: Vec<u64> = Vec::new();
    for i in 2..=l {
        if primes[i] == 1 {
            for j in i..=l {
                if i*j < l {
                    primes[i*j] = 0;
                };
            };
        }else {
            break;
        };
    };
    primes
}

#[test]
fn test_case() {
    assert_eq!(get_prime_at(6), 13);
}
