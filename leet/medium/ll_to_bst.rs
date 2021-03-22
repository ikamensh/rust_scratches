// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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
}

fn bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let mid = nums.len() / 2;
    let left = nums[..mid].to_vec();
    let right = nums[mid + 1..].to_vec();
    Some(
        Rc::new(
            RefCell::new(
                TreeNode { val: nums[mid], left: bst(left), right: bst(right) }
            )
        )
    )
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = vec![];
        let mut cur = head;
        while let Some(i) = cur {
            cur = i.next;
            v.push(i.val);
        }
        bst(v)
    }
}
