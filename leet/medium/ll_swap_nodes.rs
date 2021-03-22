// https://leetcode.com/problems/swapping-nodes-in-a-linked-list/

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head?;
        let len = {
            let mut cur = &head;
            let mut len = 1;
            while let Some(i) = &cur.next{
                cur = i;
                len += 1;
            }

            len
        };

        let mut idx_end = len - k;
        let mut k = k - 1;
        if k == idx_end {
          return Some(head)
        }
        if k > idx_end {
          let tmp = k;
          k = idx_end;
          idx_end = tmp;
        }

        let mut sentinel = Box::new(ListNode{val:0, next:Some(head)});

        let mut front = &sentinel.next;
        for i in 0..k {
          front = &front.as_ref()?.next;
        }

        let mut rear = front;
        for i in k..idx_end {
            rear = &rear.as_ref()?.next;
        }

        let v_front = rear.as_ref()?.val;
        let v_back = front.as_ref()?.val;

        let mut cur = &mut sentinel.next;
        for i in 0..k {
          cur = &mut cur.as_mut()?.next;
        }
        cur.as_mut()?.val = v_front;

        for i in k..idx_end {
          cur = &mut cur.as_mut()?.next;
        }
        cur.as_mut()?.val = v_back;

        sentinel.next
    }
}


