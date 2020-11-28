#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    // T -> (T) num (T) | X
    // 巴科斯范式（BNF）
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            let l = ["(", &self.serialize(root.borrow().left.clone()), ")"].concat();
            let r = ["(", &self.serialize(root.borrow().right.clone()), ")"].concat();
            return [l, root.borrow().val.to_string(), r].concat();
        } else {
            "X".to_string()
        }
    }

    fn parse_sub_tree(&self, chars: &[u8], ptr: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        // 跳过左括号
        *ptr += 1;
        let sub_tree = self.parse(chars, ptr);
        // 跳过右括号
        *ptr += 1;
        sub_tree
    }

    fn parse_int(&self, chars: &[u8], ptr: &mut usize) -> i32 {
        let mut x = 0;
        let mut sgn = 1;

        if !chars[*ptr].is_ascii_digit() {
            sgn = -1;
            *ptr += 1;
        }

        while chars[*ptr].is_ascii_digit() {
            x = x * 10 + (chars[*ptr] - b'0') as i32;
            *ptr += 1;
        }

        return x * sgn;
    }

    fn parse(&self, chars: &[u8], ptr: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if chars[*ptr] == b'X' {
            *ptr += 1;
            return None;
        }
        let mut cur = TreeNode::new(0);
        cur.left = self.parse_sub_tree(chars, ptr);
        cur.val = self.parse_int(chars, ptr);
        cur.right = self.parse_sub_tree(chars, ptr);
        Some(Rc::new(RefCell::new(cur)))
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ptr: usize = 0;
        let chars = data.as_bytes();
        self.parse(&chars, &mut ptr)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p297_binary_tree_t1() {
        let obj = Codec::new();
        let data: String = obj.serialize(btree![1, 2, 3]);
        assert_eq!(data, "((X)2(X))1((X)3(X))".to_string());

        let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
        assert_eq!(ans, btree![1, 2, 3]);
    }
}
