// Definition for a binary tree node.
//
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
    stack: Vec<MarkedNode>,
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

    pub fn next(&mut self) -> Option<i32> {
        match self.stack.pop() {
            Some(mut node) => {
                let v = node.node.borrow().val;
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
                } else {
                    while let Some(n) = self.stack.pop() {
                        if !n.visit {
                            self.stack.push(n);
                            break;
                        }
                    }
                }
                Some(v)
            }
            None => None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ret = Vec::new();
        let mut dfs1 = DFS::new(root1);
        let mut dfs2 = DFS::new(root2);

        let mut next1 = dfs1.next();
        let mut next2 = dfs2.next();

        loop {
            match (next1, next2) {
                (Some(v1), Some(v2)) => {
                    if v1 < v2 {
                        ret.push(v1);
                        next1 = dfs1.next();
                    } else {
                        ret.push(v2);
                        next2 = dfs2.next();
                    }
                }
                (None, Some(v2)) => {
                    ret.push(v2);
                    next2 = dfs2.next();
                }
                (Some(v1), None) => {
                    ret.push(v1);
                    next1 = dfs1.next();
                }
                (None, None) => break,
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let result = Solution::get_all_elements(root1, root2);
        assert_eq!(vec![0, 1, 1, 2, 3, 4], result);
    }
}
