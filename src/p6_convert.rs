#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
// 比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
// L   C   I   R
// E T O E S I I G
// E   D   H   N
// 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
// i ->
// L  D  R
// E OE II
// EC IH N
// T  S  G

// answers
impl Solution {
    pub fn p6_convert(s: String, num_rows: i32) -> String {
        // codes
        if num_rows == 1 {
            return s;
        }

        let chars = s.chars().collect::<Vec<char>>();
        let len = s.len();
        let mut ans = vec![];
        let num_rows = num_rows as usize;
        let g = 2 * num_rows - 2;

        for i in 0..num_rows {
            let mut k = i;
            while k < len {
                ans.push(chars[k]);
                if i != 0 && i != num_rows - 1 {
                    let next = k + 2 * num_rows - 2 * i - 2;
                    if next < len {
                        ans.push(chars[next]);
                    }
                }
                k += g;
            }
        }

        ans.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p6_convert_t1() {
        assert_eq!(
            Solution::p6_convert("LEETCODEISHIRING".to_string(), 3),
            "LCIRETOESIIGEDHN".to_string()
        );
    }

    #[test]
    fn p6_convert_t2() {
        assert_eq!(Solution::p6_convert("A".to_string(), 1), "A".to_string());
    }
}
