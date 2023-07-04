/// https://projecteuler.net/problem=2

// pub fn sum_of_even_fibonacci_numbers(limit: i32) -> i32 {
//     let mut sum = 0;
//     let mut a = 1;
//     let mut b = 1;
//     while b < limit {
//         if b & 0x01 == 0 { sum += b }
//         let c = a + b;
//         a = b;
//         b = c;
//     }
//     sum
// }

/// every third number is even in sequence fibonacci numbers
// pub fn sum_of_even_fibonacci_numbers(limit: i32) -> i32 {
//     let mut sum = 0;
//     let mut a = 1;
//     let mut b = 1;
//     let mut c = a + b;
//     while c < limit {
//         sum += c;
//         a = b + c;
//         b = c + a;
//         c = a + b;
//     }
//     sum
// }

/// every third number is even in sequence fibonacci numbers, and F(n) = F(n - 1) + F(n - 2)
/// => F(n) = 4 * F(n - 3) + F(n - 6)
pub fn sum_of_even_fibonacci_numbers(limit: i32) -> i32 {
    let mut sum = 0;
    let mut a = 0;
    let mut b = 2;
    while b < limit {
        sum += b;
        let temp = 4 * b + a;
        a = b;
        b = temp;
    }
    sum
}

#[test]
fn test_sum_of_even_fibonacci_numbers() {
    assert_eq!(sum_of_even_fibonacci_numbers(4000_000), 4613732)
}