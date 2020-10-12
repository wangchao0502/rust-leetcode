#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// two pointer exchange
impl Solution {
    pub fn p344_reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 {
            return;
        }

        // codes
        let (mut pl, mut pr) = (0, s.len() - 1);

        while pl < pr {
            s.swap(pl, pr);
            pl += 1;
            pr -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p344_reverse_string_t1() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];

        Solution::p344_reverse_string(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn p344_reverse_string_t2() {
        let mut input = vec!['h', 'o'];
        let output = vec!['o', 'h'];

        Solution::p344_reverse_string(&mut input);
        assert_eq!(input, output);
    }
}
