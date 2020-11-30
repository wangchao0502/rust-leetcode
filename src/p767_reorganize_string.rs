#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
// 基于计数的贪心算法
impl Solution {
    pub fn p767_reorganize_string(s: String) -> String {
        // codes
        if s.len() < 2 {
            return s;
        }
        let mut counts = vec![0; 26];
        let mut max_count = 0;
        let len = s.len();

        for c in s.bytes() {
            counts[(c - b'a') as usize] += 1;
            max_count = max_count.max(counts[(c - b'a') as usize]);
        }

        if max_count > (len + 1) / 2 {
            return "".to_string();
        }
        let mut reorganize_arr = vec![0 as u8; len];
        let mut even_idx = 0;
        let mut odd_idx = 1;
        let half_len = len / 2;

        for i in 0..26 {
            let c = b'a' + i as u8;
            while counts[i] > 0 && counts[i] <= half_len && odd_idx < len {
                reorganize_arr[odd_idx] = c;
                counts[i] -= 1;
                odd_idx += 2;
            }

            while counts[i] > 0 {
                reorganize_arr[even_idx] = c;
                counts[i] -= 1;
                even_idx += 2;
            }
        }

        reorganize_arr
            .iter()
            .map(|&c| c as char)
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p767_reorganize_string_t1() {
        assert_eq!(
            Solution::p767_reorganize_string("aab".to_string()),
            "aba".to_string()
        );
    }

    #[test]
    fn p767_reorganize_string_t2() {
        assert_eq!(
            Solution::p767_reorganize_string("aaab".to_string()),
            "".to_string()
        );
    }
}
