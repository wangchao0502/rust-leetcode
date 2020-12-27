#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 写一个程序，输出从 1 到 n 数字的字符串表示。
// 1. 如果 n 是3的倍数，输出“Fizz”；
// 2. 如果 n 是5的倍数，输出“Buzz”；
// 3. 如果 n 同时是3和5的倍数，输出 “FizzBuzz”。

// answers
impl Solution {
    pub fn p412_fizz_buzz(n: i32) -> Vec<String> {
        // codes
        let mut ans = vec![];

        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                ans.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                ans.push("Fizz".to_string());
            } else if i % 5 == 0 {
                ans.push("Buzz".to_string());
            } else {
                ans.push(i.to_string());
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p412_fizz_buzz_t1() {
        assert_eq!(
            Solution::p412_fizz_buzz(15),
            vec_string![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
