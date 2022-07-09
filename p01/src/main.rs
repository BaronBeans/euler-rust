/*

If we list all the natural numbers below 10 that are
multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of
these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

*/

fn main() {
    println!("{} is the answer", get_sum_multiples(1000));
}

fn get_sum_multiples(l: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..l {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[test]
fn test_case() {
    assert_eq!(get_sum_multiples(10), 23);
}

#[test]
fn answer_case() {
    assert_eq!(get_sum_multiples(1000), 233168);
}
