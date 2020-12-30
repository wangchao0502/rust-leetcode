#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 请你来实现一个 atoi 函数，使其能将字符串转换成整数。
//
// 首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。接下来的转化规则如下：
//
// 如果第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字字符组合起来，形成一个有符号整数。
// 假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成一个整数。
// 该字符串在有效的整数部分之后也可能会存在多余的字符，那么这些字符可以被忽略，它们对函数不应该造成影响。
// 注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，
// 则你的函数不需要进行转换，即无法进行有效转换。
//
// 在任何情况下，若函数不能进行有效的转换时，请返回 0 。
//
// 提示：
// 本题中的空白字符只包括空格字符 ' ' 。
// 假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231,  231 − 1]。
// 如果数值超过这个范围，请返回  INT_MAX (231 − 1) 或 INT_MIN (−231) 。

// answers
impl Solution {
    pub fn p8_my_atoi(s: String) -> i32 {
        // codes
        let mut ans: i64 = 0;
        let mut has_first = false;
        let mut sign = 1;
        for c in s.chars() {
            if !has_first {
                match c {
                    ' ' => {}
                    '-' | '+' => {
                        if c == '-' {
                            sign = -1;
                        }
                        has_first = true;
                    }
                    '0'..='9' => {
                        ans = (c as u8 - b'0') as i64;
                        has_first = true;
                    }
                    _ => return 0,
                }
            } else {
                match c {
                    '0'..='9' => {
                        ans = ans * 10 + (c as u8 - b'0') as i64;
                        if sign == 1 && ans > i32::MAX as i64 {
                            return i32::MAX;
                        } else if sign == -1 && -ans < i32::MIN as i64 {
                            return i32::MIN;
                        }
                    }
                    _ => break,
                }
            }
        }

        ans as i32 * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p8_my_atoi_t1() {
        assert_eq!(Solution::p8_my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn p8_my_atoi_t2() {
        assert_eq!(Solution::p8_my_atoi("42".to_string()), 42);
    }

    #[test]
    fn p8_my_atoi_t3() {
        assert_eq!(Solution::p8_my_atoi("42 with words".to_string()), 42);
    }

    #[test]
    fn p8_my_atoi_t4() {
        assert_eq!(Solution::p8_my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn p8_my_atoi_t5() {
        // 数字 "-91283472332" 超过 32 位有符号整数范围, 因此返回 INT_MIN
        assert_eq!(
            Solution::p8_my_atoi("-91283472332".to_string()),
            -2147483648
        );
    }

    #[test]
    fn p8_my_atoi_t6() {
        assert_eq!(
            Solution::p8_my_atoi("9223372036854775808".to_string()),
            2147483647
        );
    }
}
