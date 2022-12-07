use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

fn tree_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        None => 0,
        Some(ref n) => {
            let b = n.borrow();
            b.val + tree_sum(&b.right) + tree_sum(&b.left)
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut cur = root;
        while let Some(cur_node) = cur {
            let cloned = cur_node.clone();
            let mut node = cloned.borrow_mut();
            if low > node.val {
                cur = node.right.take();
            } else if high < node.val {
                cur = node.left.take();
            } else {
                cur = Some(cur_node);
                break;
            }
        }

        if let Some(node) = cur {
            let n = node.borrow();
            let mut sum = n.val;

            let mut cur_low = n.left.clone();
            while cur_low.is_some() {
                let n = cur_low.unwrap();
                let mut node = n.borrow_mut();
                if low < node.val {
                    sum += node.val + tree_sum(&node.right);
                    cur_low = node.left.take();
                } else if low > node.val {
                    cur_low = node.right.take();
                } else {
                    sum += node.val + tree_sum(&node.right);
                    break;
                }
            }

            let mut cur_high = n.right.clone();
            while cur_high.is_some() {
                let n = cur_high.unwrap();
                let mut node = n.borrow_mut();
                if high > node.val {
                    sum += node.val + tree_sum(&node.left);
                    cur_high = node.right.take();
                } else if high < node.val {
                    cur_high = node.left.take();
                } else {
                    sum += node.val + tree_sum(&node.left);
                    break;
                }
            }

            sum
        } else {
            0
        }
    }
}
