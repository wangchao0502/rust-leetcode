#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

pub const NULL: i32 = std::i32::MIN;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn to_string_internal(&self, depth: i32) -> String {
        let mut s = format!("{{ v: {},", self.val);
        s += "\n";

        for _ in 0..depth {
            s += "  ";
        }
        s += "l: ";
        if self.left.is_none() {
            s += "None";
        } else {
            s += self
                .left
                .clone()
                .unwrap()
                .borrow()
                .to_string_internal(depth + 1)
                .as_str();
        }
        s += "\n";
        for _ in 0..depth {
            s += "  ";
        }
        s += "r: ";
        if self.right.is_none() {
            s += "None";
        } else {
            s += self
                .right
                .clone()
                .unwrap()
                .borrow()
                .to_string_internal(depth + 1)
                .as_str();
        }
        s += "\n";
        for _ in 0..depth {
            s += "  ";
        }
        s += "}}";
        s
    }

    pub fn to_string(&self) -> String {
        self.to_string_internal(0)
    }
}

/**
 * 构造树的时候,按照leetcode的规则来, 如果父节点是空,那么就不必给出子节点.
 * 这样就不是严格意义上的那个2i+1, 2i+2, 稍微复杂一点
 */
pub fn build_tree_ignore_parent(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let parent = Vec::new();
    return buld_tree_ignore_parent_internal(v, &parent, 0);
}

fn buld_tree_ignore_parent_internal(
    v: &Vec<i32>,
    parent: &Vec<Rc<RefCell<TreeNode>>>,
    from: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    let this_level_len = parent.len();
    if this_level_len == 0 && v.len() == 0 {
        return None; //空树
    } else if this_level_len == 0 {
        let root = Rc::new(RefCell::new(TreeNode {
            val: v[0],
            left: None,
            right: None,
        }));
        let parent = vec![root.clone()];
        buld_tree_ignore_parent_internal(v, &parent, 1);
        return Some(root);
    } else {
        //其他层
        let mut has_left = true;
        let mut this_level = v.iter().skip(from);
        let mut next_level = Vec::new();

        parent.iter().for_each(|r| {
            if !has_left {
                return;
            }
            //处理左右子节点
            let mut cnt = 0;
            loop {
                if cnt >= 2 {
                    break;
                }
                cnt += 1;
                if let Some(c1) = this_level.next() {
                    //如果为空,相当于已经添加到树上了
                    if *c1 != NULL {
                        let n = Rc::new(RefCell::new(TreeNode {
                            val: *c1,
                            left: None,
                            right: None,
                        }));
                        next_level.push(n.clone());
                        if cnt == 1 {
                            r.borrow_mut().left = Some(n);
                        } else {
                            r.borrow_mut().right = Some(n);
                        }
                    }
                } else {
                    has_left = false;
                }
            }
        });
        if has_left {
            buld_tree_ignore_parent_internal(v, &next_level, from + parent.len() * 2);
        }
        return None;
    }
}

pub fn build_tree(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_helper(0, v)
}

fn build_tree_helper(i: usize, v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if i >= v.len() {
        return None;
    }
    if v[i] == NULL {
        return None;
    }
    let left = build_tree_helper(2 * i + 1, v);
    let right = build_tree_helper(2 * i + 2, v);
    Some(Rc::new(RefCell::new(TreeNode {
        val: v[i],
        left,
        right,
    })))
}