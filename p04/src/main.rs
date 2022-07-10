/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
fn main() {
    let mut max: u64 = 0;
    for x in 100..1000 {
        for y in 100..1000 {
            let prod = x * y;
            let pal = is_palindrome(&prod.to_string());
            if pal {
                if prod > max {
                    max = prod;
                };
            };
        };
    };
    println!("{}", max);
}

fn is_palindrome(input: &str) -> bool {
    let forwards: Vec<char> = input.chars().collect();
    let backwards: Vec<char> = input.chars().rev().collect();
    backwards == forwards
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome("101"), true);
    assert_eq!(is_palindrome("1001"), true);
    assert_eq!(is_palindrome("10101"), true);
    assert_eq!(is_palindrome("99999"), true);

    assert_eq!(is_palindrome("99998"), false);
    assert_eq!(is_palindrome("1234"), false);
    assert_eq!(is_palindrome("1234"), false);
}
