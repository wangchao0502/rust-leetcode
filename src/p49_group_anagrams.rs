#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。

// answers
impl Solution {
    pub fn p49_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // codes
        // strs.into_iter()
        //     .fold(std::collections::BTreeMap::new(), |mut map, s| {
        //         let mut key = std::collections::BTreeMap::new();
        //         s.as_bytes()
        //             .iter()
        //             .for_each(|&c| *key.entry(c).or_insert(0) += 1);
        //         map.entry(key).or_insert(vec![]).push(s);
        //         map
        //     })
        //     .into_iter()
        //     .map(|(_, v)| v)
        //     .collect()
        let mut group = std::collections::HashMap::new();
        for s in strs.into_iter() {
            let mut counter = [0u8; 26];
            for &byte in s.as_bytes() {
                counter[(byte - b'a') as usize] += 1;
            }
            group.entry(counter).or_insert(Vec::new()).push(s)
        }
        group.into_iter().map(|(_k, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::{assert_eq_sorted, vec_string};

    #[test]
    fn p49_group_anagrams_t1() {
        assert_eq_sorted!(
            Solution::p49_group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]),
            vec![
                vec_string!["eat", "tea", "ate"],
                vec_string!["tan", "nat"],
                vec_string!["bat"]
            ]
        );
    }
}
