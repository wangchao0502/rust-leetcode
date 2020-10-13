#![allow(dead_code)]

// use mods
use crate::utils::list_node::ListNode;

pub struct Solution {}

// add structs

// answers
// recursion, swap
impl Solution {
    pub fn p24_swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // codes
        head.and_then(|mut n| {
            match n.next {
                Some(mut m) => {
                    n.next = Self::p24_swap_pairs(m.next);
                    m.next = Some(n);
                    Some(m)
                },
                None => Some(n)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::list_node::*;

    #[test]
    fn p24_swap_pairs_t1() {
        let input = build_list_node(&vec![1, 2, 3, 4]);
        let output = Solution::p24_swap_pairs(input);
        assert_eq!(output, build_list_node(&vec![2, 1, 4, 3]));
    }
}

