/// https://projecteuler.net/problem=1

// pub fn sum_of_multiples_3_and_5_below_1000() -> i32 {
//     let mut sum = 0;
//     for i in 0..1000 {
//         if i % 3 == 0 || i % 5 == 0 {
//             sum += i;
//         }
//     }
//     sum
// }

pub fn sum_of_multiples_3_and_5_below_1000() -> i32 {
    sum(1000, 3) + sum(1000, 5) - sum(1000, 3 * 5)
}

fn sum(below: i32, i: i32) -> i32 {
    let count = (below - 1) / i;
    i * count * (count + 1) >> 1
}

#[test]
fn test_sum_of_multiples_3_and_5_below_1000() {
    assert_eq!(sum_of_multiples_3_and_5_below_1000(), 233168)
}