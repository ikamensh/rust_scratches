// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head?;
        let idx_remove : usize = {
            let mut cur = &head;
            let mut len = 1;
            while let Some(i) = &cur.next{
                cur = i;
                len += 1;
            }

            len - n as usize
        };

        if idx_remove == 0 {
            return head.next;
        } else if idx_remove == 1 {
            head.next = head.next?.next;
            return Some(head);
        }

        let mut cur = &mut head;
        let mut len = 1;
        while let Some(i) = &mut cur.next{
            cur = i;
            len += 1;
            if len == idx_remove {
                cur.next = cur.next.take()?.next;
                break;
            }
        }

        Some(head)
    }
}


