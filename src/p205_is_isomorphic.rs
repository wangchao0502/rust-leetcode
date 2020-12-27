#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定两个字符串 s 和 t，判断它们是否是同构的。
// 如果 s 中的字符可以被替换得到 t ，那么这两个字符串是同构的。
// 所有出现的字符都必须用另一个字符替换，同时保留字符的顺序。两个字符不能映射到同一个字符上，但字符可以映射自己本身。

// answers
// LINK -> p290
impl Solution {
    pub fn p205_is_isomorphic(s: String, t: String) -> bool {
        // codes
        use std::collections::HashMap;
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let n = s.len();
        let ss = &s[..];
        let tt = &t[..];

        for i in 0..n {
            let a = map1.entry(&ss[i..i + 1]).or_insert(&tt[i..i + 1]);
            let b = map2.entry(&tt[i..i + 1]).or_insert(&ss[i..i + 1]);
            if &ss[i..i + 1] != *b || &tt[i..i + 1] != *a {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p205_is_isomorphic_t1() {
        assert_eq!(
            Solution::p205_is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
    }

    #[test]
    fn p205_is_isomorphic_t2() {
        assert_eq!(
            Solution::p205_is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
    }

    #[test]
    fn p205_is_isomorphic_t3() {
        assert_eq!(
            Solution::p205_is_isomorphic("ab".to_string(), "aa".to_string()),
            false
        );
    }
}
