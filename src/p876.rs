#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    #[allow(dead_code)]
    fn new_with_next(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
}

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = head.as_ref().expect("head must have value");
        let mut middle = head.as_ref().expect("head must have value");

        loop {
            match tail.next.as_ref() {
                Some(next) => {
                    middle = &middle.next.as_ref().expect("middle must have next");
                    match next.next.as_ref() {
                        Some(two_next) => {
                            tail = two_next;
                        }
                        None => break,
                    }
                }
                None => break,
            };
        }
        Some(middle.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let six = Some(Box::new(ListNode::new(6)));
        let five = Some(Box::new(ListNode::new_with_next(5, six)));
        let four = Some(Box::new(ListNode::new_with_next(4, five)));
        let three = Some(Box::new(ListNode::new_with_next(3, four)));
        let two = Some(Box::new(ListNode::new_with_next(2, three)));
        let one = Some(Box::new(ListNode::new_with_next(1, two)));
        assert_eq!(4, Solution::middle_node(one).unwrap().val)
    }

    #[test]
    fn it_works2() {
        let five = Some(Box::new(ListNode::new(5)));
        let four = Some(Box::new(ListNode::new_with_next(4, five)));
        let three = Some(Box::new(ListNode::new_with_next(3, four)));
        let two = Some(Box::new(ListNode::new_with_next(2, three)));
        let one = Some(Box::new(ListNode::new_with_next(1, two)));
        assert_eq!(3, Solution::middle_node(one).unwrap().val)
    }
}
