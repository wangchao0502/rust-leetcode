#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 字符串的左旋转操作是把字符串前面的若干个字符转移到字符串的尾部。
// 请定义一个函数实现字符串左旋转操作的功能。比如，输入字符串"abcdefg"和数字2，
// 该函数将返回左旋转两位得到的结果"cdefgab"。

// answers
impl Solution {
    pub fn offer58_reverse_left_words(s: String, n: i32) -> String {
        // codes
        let n = n as usize;
        let mut ans = vec![];
        let chars = s.chars().collect::<Vec<char>>();

        for i in n..(s.len() + n) {
            ans.push(chars[i % s.len()]);
        }

        ans.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offer58_reverse_left_words_t1() {
        assert_eq!(
            Solution::offer58_reverse_left_words("abcdefg".to_string(), 2),
            "cdefgab".to_string()
        );
    }
}
