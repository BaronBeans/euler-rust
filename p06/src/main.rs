/*
The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 +...+ 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 +...+ 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn main() {
    println!("{}", find_answer(100));
}

fn find_answer(l: u64) -> u64 {
    let sum_of_squares = sum_of_square(l);
    let square_of_sums = square_of_sum(l);

    square_of_sums - sum_of_squares
}

fn sum_of_square(l: u64) -> u64 {
    let mut count: u64 = 0;
    for i in 1..=l {
        count += i*i;
    };
    count
}

fn square_of_sum(l: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=l {
        sum += i;
    };
    sum * sum
}

#[test]
fn test_sum_of_square() {
    assert_eq!(sum_of_square(10), 385);
}

#[test]
fn test_square_of_sum() {
    assert_eq!(square_of_sum(10), 3025);
}

#[test]
fn test_case() {
    assert_eq!(find_answer(10), 2640)
}

#[test]
fn answer_case() {
    assert_eq!(find_answer(100), 25164150)
}
