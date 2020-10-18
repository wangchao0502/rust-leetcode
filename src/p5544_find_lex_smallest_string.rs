#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// You are given a string s of even length consisting of digits from 0 to 9, and two integers a and b.
//
// You can apply either of the following two operations any number of times and in any order on s:
//
// Add a to all odd indices of s (0-indexed). Digits post 9 are cycled back to 0. For example,
// if s = "3456" and a = 5, s becomes "3951".
// Rotate s to the right by b positions.
// For example, if s = "3456" and b = 1, s becomes "6345".
// Return the lexicographically smallest string you can obtain by applying the above operations
// any number of times on s.
//
// A string a is lexicographically smaller than a string b (of the same length) if in the first
// position where a and b differ, string a has a letter that appears earlier in the alphabet than
// the corresponding letter in b. For example, "0158" is lexicographically smaller than "0190"
// because the first position they differ is at the third letter, and '5' comes before '9'.

// enumerate
impl Solution {
    // 最大公约数
    fn gcd(x: i32, y: i32) -> i32 {
        if y == 0 {
            x
        } else {
            Self::gcd(y, x % y)
        }
    }

    pub fn p5544_find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        // codes
        let n = s.len();
        let ss = s.repeat(2);
        let mut ans = s.clone();
        // 轮转移动步数
        let g = Self::gcd(n as i32, b);

        for i in (0..n).step_by(g as usize) {
            let p = &ss[i..(i + n)];
            println!("{:?}", p);
            // 叠加计算10次回复原样
            for j in 0..10 {
                let th = if g % 2 == 0 { 1 } else { 10 };
                // 如果轮转步数为奇数，可以对奇数位和偶数位同时操作
                // 如果轮转步数为偶数，只能对偶数位进行操作
                for k in 0..th {
                    let mut q = p.clone().to_string();
                    let mut bytes = q.into_bytes();
                    // '0' is 48
                    for t in (1..n).step_by(2) {
                        bytes[t] = (48 + (bytes[t] as i32 - 48 + a * j) % 10) as u8;
                    }
                    for t in (0..n).step_by(2) {
                        bytes[t] = (48 + (bytes[t] as i32 - 48 + a * k) % 10) as u8;
                    }
                    unsafe { q = String::from_utf8_unchecked(bytes) }

                    println!("new string -> {}", q);
                    ans = ans.min(q);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5544_find_lex_smallest_string_t1() {
        assert_eq!(
            Solution::p5544_find_lex_smallest_string("5525".to_string(), 9, 2),
            "2050".to_string()
        );
    }

    #[test]
    fn p5544_find_lex_smallest_string_t2() {
        assert_eq!(
            Solution::p5544_find_lex_smallest_string("0011".to_string(), 4, 2),
            "0011".to_string()
        );
    }

    #[test]
    fn p5544_find_lex_smallest_string_t3() {
        assert_eq!(
            Solution::p5544_find_lex_smallest_string("12345".to_string(), 3, 3),
            "00224".to_string()
        );
    }

    #[test]
    fn p5544_find_lex_smallest_string_t4() {
        assert_eq!(
            Solution::p5544_find_lex_smallest_string("123456".to_string(), 3, 3),
            "002244".to_string()
        );
    }

    #[test]
    fn p5544_find_lex_smallest_string_t5() {
        assert_eq!(
            Solution::p5544_find_lex_smallest_string("123456".to_string(), 3, 2),
            "103254".to_string()
        );
    }
}
