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

    #[inline]
    #[allow(dead_code)]
    fn from_vec(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut tail = None;
        for v in vals.iter().rev() {
            let node = ListNode {
                val: *v,
                next: tail,
            };
            tail = Some(Box::new(node));
        }
        tail
    }
}

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut odd = Box::new(ListNode::new(-1));
        let mut cur_odd = &mut odd;
        let mut even = Box::new(ListNode::new(-1));
        let mut cur_even = &mut even;
        let mut is_odd = true;
        while head.is_some() {
            if is_odd {
                cur_odd.next = head;
                cur_odd = cur_odd.next.as_mut()?;
                head = cur_odd.next.take();
            } else {
                cur_even.next = head;
                cur_even = cur_even.next.as_mut()?;
                head = cur_even.next.take();
            }
            is_odd = !is_odd;
        }
        cur_odd.next = even.next;
        return odd.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            ListNode::from_vec(vec![1, 3, 5, 2, 4]),
            Solution::odd_even_list(list)
        );
    }

    #[test]
    fn it_works2() {
        let list = ListNode::from_vec(vec![2, 1, 3, 5, 6, 4, 7]);
        assert_eq!(
            ListNode::from_vec(vec![2, 3, 6, 7, 1, 5, 4]),
            Solution::odd_even_list(list)
        );
    }
}
