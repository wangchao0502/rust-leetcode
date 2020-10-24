#![allow(dead_code)]

// use mods
use crate::utils::list_node::ListNode;

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p234_is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // codes
        let mut arr = Vec::new();
        let mut head = head;

        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        let (mut p1, mut p2) = (0, arr.len());

        while p1 < p2 {
            if arr[p1] != arr[p2 - 1] {
                return false;
            }
            p1 += 1;
            p2 -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::list_node::build_list_node;

    #[test]
    fn p234_is_palindrome_t1() {
        assert_eq!(
            Solution::p234_is_palindrome(build_list_node(&vec![1, 2, 2, 1])),
            true
        );
    }

    #[test]
    fn p234_is_palindrome_t2() {
        assert_eq!(
            Solution::p234_is_palindrome(build_list_node(&vec![1, 2])),
            false
        );
    }

    #[test]
    fn p234_is_palindrome_t3() {
        assert_eq!(
            Solution::p234_is_palindrome(build_list_node(&vec![1, 2, 3, 2, 1])),
            true
        );
    }
}
