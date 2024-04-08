// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}

// solution

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|head| {
            let mut fast: *const ListNode = &*head;
            let mut slow = head;

            while let Some(next_fast) = &unsafe { &*fast }.next {
                fast = &**next_fast.next.as_ref().unwrap_or(next_fast);
                slow = slow.next.unwrap();
            }

            slow
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn simple_1_2_3_4_5() {
        let node5 = ListNode { val: 5, next: None };
        let node4 = ListNode {
            val: 4,
            next: Some(Box::new(node5)),
        };
        let node3 = ListNode {
            val: 3,
            next: Some(Box::new(node4)),
        };
        let node2 = ListNode {
            val: 2,
            next: Some(Box::new(node3.clone())),
        };
        let node1 = ListNode {
            val: 1,
            next: Some(Box::new(node2)),
        };

        assert_eq!(
            Solution::middle_node(Some(Box::new(node1))),
            Some(Box::new(node3))
        );
    }
}
