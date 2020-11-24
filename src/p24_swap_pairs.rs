#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// add structs

// answers
// recursion, swap
impl Solution {
    pub fn p24_swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // codes
        head.and_then(|mut n| match n.next {
            Some(mut m) => {
                n.next = Self::p24_swap_pairs(m.next);
                m.next = Some(n);
                Some(m)
            }
            None => Some(n),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p24_swap_pairs_t1() {
        let input = linkedlist![1, 2, 3, 4];
        let output = Solution::p24_swap_pairs(input);
        assert_eq!(output, linkedlist![2, 1, 4, 3]);
    }

    #[test]
    fn p24_swap_pairs_t2() {
        let input = linkedlist![];
        let output = Solution::p24_swap_pairs(input);
        assert_eq!(output, linkedlist![]);
    }

    #[test]
    fn p24_swap_pairs_t3() {
        let input = linkedlist![1];
        let output = Solution::p24_swap_pairs(input);
        assert_eq!(output, linkedlist![1]);
    }

    #[test]
    fn p24_swap_pairs_t4() {
        let input = linkedlist![1, 2, 3, 4, 5];
        let output = Solution::p24_swap_pairs(input);
        assert_eq!(output, linkedlist![2, 1, 4, 3, 5]);
    }
}
