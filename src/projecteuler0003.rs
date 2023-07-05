// solution1: 可以处理 n = 1 的情况
pub fn largest_prime_factor(mut n: i64) -> i64 {
    let mut i: i64 = 1;
    while i * i < n {
        i += 1;
        while n % i == 0 {
            n /= i;
        }
    }
    if n == 1 { i } else { n }
}
// solution2: solution1 的变种，可以处理 n = 1 的情况
// pub fn largest_prime_factor(mut n: i64) -> i64 {
//     let mut i: i64 = 2;
//     while i * i <= n {
//         while n % i == 0 && n != i {
//             n /= i;
//         }
//         i += 1;
//     }
//     n
// }

// solution3: 不能处理 n = 1 的情况
// https://zhuanlan.zhihu.com/p/25672924
// pub fn largest_prime_factor(mut n: i64) -> i64 {
//     let mut i: i64 = 2;
//     while n != 1 {
//         if n % i == 0 {
//             n /= i;
//         } else {
//             i += 1;
//         }
//     }
//     if n == 1 { i } else { n }
// }

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(6008_5147_5143), 6857);
    assert_eq!(largest_prime_factor(37 * 37), 37);
    assert_eq!(largest_prime_factor(31 * 37), 37);
    assert_eq!(largest_prime_factor(37), 37);
    assert_eq!(largest_prime_factor(3), 3);
    assert_eq!(largest_prime_factor(2), 2);
    assert_eq!(largest_prime_factor(1), 1);
}