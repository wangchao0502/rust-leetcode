#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。
// 字符          数值
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。
// 27 写做  XXVII, 即为 XX + V + II 。
// 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。
// 数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。
// 同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
// I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
// X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
// C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
// 给定一个整数，将其转为罗马数字。输入确保在 1 到 3999 的范围内。

// answers
impl Solution {
    pub fn p12_int_to_roman(num: i32) -> String {
        // codes
        // 1-3 XXX -> 4 XY -> 5 Y -> 6-7 YXX -> 8-9 -> XXZ
        let dict = vec![
            vec!['I', 'V', 'X'],
            vec!['X', 'L', 'C'],
            vec!['C', 'D', 'M'],
            vec!['M'],
        ];
        let mut ans: Vec<char> = vec![];
        // 1-9 -> 1, 10-99 -> 2, 100-999 -> 3
        let mut base_pow = 1;
        let mut x = num;

        while x >= 10 {
            base_pow += 1;
            x /= 10;
        }

        x = num;

        // 1994 -> 4
        // 1004
        while x > 0 {
            let base = 10_i32.pow(base_pow - 1);
            let group = &dict[base_pow as usize - 1];
            let n = x / base;
            match n {
                1 | 2 | 3 => {
                    for _ in 1..=n {
                        ans.push(group[0]);
                    }
                }
                4 => {
                    ans.push(group[0]);
                    ans.push(group[1]);
                }
                5 => {
                    ans.push(group[1]);
                }
                6 | 7 | 8 => {
                    ans.push(group[1]);
                    for _ in 6..=n {
                        ans.push(group[0]);
                    }
                }
                9 => {
                    ans.push(group[0]);
                    ans.push(group[2]);
                }
                _ => {
                    // 0 -> do nothing
                }
            }
            // println!("{}:{:?}", n, ans);
            base_pow -= 1;
            x -= n * base;
        }

        ans.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p12_int_to_roman_t1() {
        assert_eq!(Solution::p12_int_to_roman(3), "III".to_string());
    }

    #[test]
    fn p12_int_to_roman_t2() {
        assert_eq!(Solution::p12_int_to_roman(4), "IV".to_string());
    }

    #[test]
    fn p12_int_to_roman_t3() {
        assert_eq!(Solution::p12_int_to_roman(1994), "MCMXCIV".to_string());
    }

    #[test]
    fn p12_int_to_roman_t4() {
        assert_eq!(Solution::p12_int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn p12_int_to_roman_t5() {
        assert_eq!(Solution::p12_int_to_roman(10), "X".to_string());
    }
}
