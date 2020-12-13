#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 如果一个十进制数字不含任何前导零，且每一位上的数字不是 0 就是 1 ，那么该数字就是一个 十-二进制数 。
// 例如，101 和 1100 都是 十-二进制数，而 112 和 3001 不是。
// 给你一个表示十进制整数的字符串 n ，返回和为 n 的 十-二进制数 的最少数目。

// 输入：n = "32"
// 输出：3
// 解释：10 + 11 + 11 = 32

// answers
// 智力测试
impl Solution {
    pub fn p5626_min_partitions(n: String) -> i32 {
        // codes
        (n.bytes().max().unwrap() - b'0') as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5626_min_partitions_t1() {
        assert_eq!(Solution::p5626_min_partitions("32".to_string()), 3);
    }

    #[test]
    fn p5626_min_partitions_t2() {
        assert_eq!(Solution::p5626_min_partitions("82734".to_string()), 8);
    }

    #[test]
    fn p5626_min_partitions_t3() {
        assert_eq!(
            Solution::p5626_min_partitions("27346209830709182346".to_string()),
            9
        );
    }
}
