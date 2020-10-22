#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p763_partition_labels(s: String) -> Vec<i32> {
        // codes
        let mut last = [0; 26];
        for (i, c) in s.chars().enumerate() {
            last[c as usize - 97] = i;
        }

        let mut ans = vec![];
        let mut start: usize = 0;
        let mut end: usize = 0;

        for (i, c) in s.chars().enumerate() {
            end = end.max(last[c as usize - 97]);
            if i == end {
                ans.push((end - start + 1) as i32);
                start = end + 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p763_partition_labels_t1() {
        assert_eq!(
            Solution::p763_partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }
}
