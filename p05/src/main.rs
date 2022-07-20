/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/
fn main() {
    find_number_10();
    println!("{}", find_number_20());
}

fn find_number_10() -> u64 {
    let mut i: u64 = 1;
    while i % 10 != 0
        || i % 9 != 0
        || i % 8 != 0
        || i % 7 != 0
        || i % 6 != 0
        || i % 5 != 0
        || i % 4 != 0
        || i % 3 != 0
        || i % 2 != 0
        || i % 1 != 0
    {
        i += 1;
    }
    i
}

fn find_number_20() -> u64 {
    let mut i: u64 = 1;
    while i % 20 != 0
        || i % 19 != 0
        || i % 18 != 0
        || i % 17 != 0
        || i % 16 != 0
        || i % 15 != 0
        || i % 14 != 0
        || i % 13 != 0
        || i % 12 != 0
        || i % 11 != 0
        || i % 10 != 0
        || i % 9 != 0
        || i % 8 != 0
        || i % 7 != 0
        || i % 6 != 0
        || i % 5 != 0
        || i % 4 != 0
        || i % 3 != 0
        || i % 2 != 0
        || i % 1 != 0
    {
        i += 1;
    }
    i
}

#[test]
fn test_case() {
    assert_eq!(find_number_10(), 2520);
}
