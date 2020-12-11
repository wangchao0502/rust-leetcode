#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given a valid (IPv4) IP address, return a defanged version of that IP address.
// A defanged IP address replaces every period "." with "[.]".

// answers
impl Solution {
    pub fn p1108_defang_i_paddr(address: String) -> String {
        // codes
        address.replace(".", "[.]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1108_defang_i_paddr_t1() {
        assert_eq!(
            Solution::p1108_defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
    }
}
