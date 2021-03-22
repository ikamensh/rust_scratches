// https://leetcode.com/problems/diameter-of-binary-tree/

pub struct Solution {}

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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::collections::{HashMap, VecDeque};

type Node = Option<Rc<RefCell<TreeNode>>>;


fn find_depth(maybe_node: &Node, depth_map: &mut HashMap<i32, u32>) -> u32 {
    match maybe_node {
        None => {return 0;}
        Some(node) => {
            if depth_map.contains_key(&node.borrow().val) {return depth_map.get(&node.borrow().val).unwrap().clone()}

            let d_left = find_depth(&node.borrow().left, depth_map);
            let d_right = find_depth(&node.borrow().right, depth_map);
            let depth = 1 + u32::max(d_left, d_right);
            depth_map.insert(node.borrow().val, depth);
            return depth
        }
    }
}

fn diameter_through(node: &Rc<RefCell<TreeNode>>, depth_map: &mut HashMap<i32, u32>) -> u32{
        find_depth(&node.borrow().left, depth_map)  +
        find_depth(&node.borrow().right, depth_map)
}


fn enumerate(node: &Rc<RefCell<TreeNode>>) {
    let mut number = 0;
    let mut stack = vec![Rc::clone(node)];
    while ! stack.is_empty(){
        let node = stack.pop().unwrap();
        node.borrow_mut().val = number;
        number += 1;
        if node.borrow().left.is_some(){
            let child = Rc::clone(node.borrow().left.as_ref().unwrap());
            stack.push(child);
        }
        if node.borrow().right.is_some(){
            let child = Rc::clone(node.borrow().right.as_ref().unwrap());
            stack.push(child);
        }
    }
}


impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        if root.is_none() {
            return 0
        }

        enumerate(&root.as_ref().unwrap());

        let mut depth = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back( Rc::clone(&root.unwrap()) );
        let mut result = 0;
        while !queue.is_empty() {
            let n = queue.pop_front();
            let n = n.unwrap();
            result = u32::max(result, diameter_through(&n, &mut depth));

            match n.borrow().left {
                None => {},
                Some(ref child) => {
                    queue.push_back(Rc::clone(child));
                }
            };

            match n.borrow().right {
                None => {},
                Some(ref child) => {
                    queue.push_back(Rc::clone(child));
                }
            };
        }

        result as i32
    }
}
