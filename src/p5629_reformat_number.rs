#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个字符串形式的电话号码 number 。number 由数字、空格 ' '、和破折号 '-' 组成。
//
// 请你按下述方式重新格式化电话号码。
//
// 首先，删除 所有的空格和破折号。
// 其次，将数组从左到右 每 3 个一组 分块，直到 剩下 4 个或更少数字。剩下的数字将按下述规定再分块：
// 2 个数字：单个含 2 个数字的块。
// 3 个数字：单个含 3 个数字的块。
// 4 个数字：两个分别含 2 个数字的块。
// 最后用破折号将这些块连接起来。注意，重新格式化过程中 不应该 生成仅含 1 个数字的块，并且 最多 生成两个含 2 个数字的块。
//
// 返回格式化后的电话号码。

// answers
impl Solution {
    pub fn p5629_reformat_number(number: String) -> String {
        // codes
        let mut chars = vec![];

        for c in number.chars() {
            if c != ' ' && c != '-' {
                chars.push(c);
            }
        }

        let len = chars.len();
        let mut ans = vec![];
        let end = if len % 3 == 1 {
            len - 4
        } else if len % 3 == 2 {
            len - 2
        } else {
            len
        };

        for i in 0..end {
            ans.push(chars[i]);
            if (i + 1) % 3 == 0 && i != len - 1 {
                ans.push('-');
            }
        }

        if len - end == 4 {
            ans.push(chars[end]);
            ans.push(chars[end + 1]);
            ans.push('-');
            ans.push(chars[end + 2]);
            ans.push(chars[end + 3]);
        } else if len - end == 2 {
            ans.push(chars[end]);
            ans.push(chars[end + 1]);
        }

        ans.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5629_reformat_number_t1() {
        assert_eq!(
            Solution::p5629_reformat_number("123 4-567".to_string()),
            "123-45-67".to_string()
        );
    }

    #[test]
    fn p5629_reformat_number_t2() {
        assert_eq!(
            Solution::p5629_reformat_number("--17-5 229 35-39475 ".to_string()),
            "175-229-353-94-75".to_string()
        );
    }

    #[test]
    fn p5629_reformat_number_t3() {
        assert_eq!(
            Solution::p5629_reformat_number("1-23-45 6".to_string()),
            "123-456".to_string()
        );
    }

    #[test]
    fn p5629_reformat_number_t4() {
        assert_eq!(
            Solution::p5629_reformat_number("123 4-5678".to_string()),
            "123-456-78".to_string()
        );
    }
}
