#![allow(unused_variables)]

use crate::datastruct::bin_treenode::TreeNode;

pub struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::inorder(root)
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];

        let mut node = root;
        loop {
            while let Some(x) = node {
                stack.push(x.clone());
                node = x.borrow().left.clone();
            }
            if stack.is_empty() {
                break;
            }
            if let Some(x) = stack.pop() {
                ans.push(x.borrow().val);
                node = x.borrow().right.clone();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], vec![9, 3, 15, 20, 7]),
            (vec![], vec![]),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::inorder_traversal(root);
            assert_eq!(ans, answer)
        }
    }
}
