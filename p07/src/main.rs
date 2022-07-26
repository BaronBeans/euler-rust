/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

use std::usize;

fn main() {
    // println!("{}", get_prime_at(10));
}

// fn get_prime_at(i: u64) -> u64 {
//
// }

fn get_primes(l: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    for i in 2..=l {
        let i_num: usize = i as usize;
        if primes[i_num] == 1 {
            for j in i..=l {
                if i * j < l {
                    let j_num: usize = j as usize;
                    primes[i_num * j_num] = 0;
                };
            }
        } else {
            break;
        };
    }
    primes
}


#[test]
fn test_get_primes() {
    assert_eq!(get_primes(6), []);
}
// #[test]
// fn test_case() {
//     assert_eq!(get_prime_at(6), 13);
// }
