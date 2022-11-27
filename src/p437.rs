// Definition for a binary tree node.
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

#[allow(dead_code)]
struct Solution {}

type Node = Rc<RefCell<TreeNode>>;

struct MarkedNode {
    pub visit: bool,
    pub node: Node,
}

struct DFS {
    pub stack: Vec<MarkedNode>,
}

impl DFS {
    pub fn new(root: Option<Node>) -> DFS {
        let mut stack = Vec::new();
        let mut t = root;
        while let Some(node) = t {
            stack.push(MarkedNode {
                visit: false,
                node: node.clone(),
            });
            t = node.borrow().left.clone()
        }
        DFS { stack }
    }

    pub fn next(&mut self) -> bool {
        match self.stack.pop() {
            Some(mut node) => {
                let mut t = node.node.borrow().right.clone();
                if t.is_some() {
                    node.visit = true;
                    self.stack.push(node);

                    while let Some(n) = t {
                        self.stack.push(MarkedNode {
                            visit: false,
                            node: n.clone(),
                        });
                        t = n.borrow().left.clone()
                    }
                    true
                } else {
                    while let Some(n) = self.stack.pop() {
                        if !n.visit {
                            self.stack.push(n);
                            break;
                        }
                    }
                    true
                }
            }
            None => false,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut dfs = DFS::new(root);
        let mut count = 0;
        loop {
            let mut sum: i64 = 0;

            for n in dfs.stack.iter().rev() {
                sum += n.node.borrow().val as i64;
                if sum == target_sum as i64 {
                    count += 1
                }
            }
            if !dfs.next() {
                break;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = Solution::path_sum(root, 8);
        assert_eq!(3, result)
    }
    // [,,,,,,1000000000]

    #[test]
    fn it_works2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1000000000,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1000000000,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 294967296,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1000000000,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1000000000,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1000000000,
                                left: None,
                                right: None,
                            }))),
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        })));
        let result = Solution::path_sum(root, 0);
        assert_eq!(0, result)
    }
}
