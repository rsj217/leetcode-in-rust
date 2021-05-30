#![allow(unused_variables)]

use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}


impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::preorder_dfs(root)
    }

    // fn preorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut ans = vec![];
    //     let mut stack = vec![];
    //     let mut node = root;
    //
    //     loop {
    //         while let Some(x) = node {
    //             ans.push(x.borrow().val);
    //             stack.push(x.clone());
    //             node = x.borrow().left.clone();
    //         }
    //         if stack.is_empty() {
    //             break;
    //         }
    //         node = stack.pop().unwrap().borrow().right.clone();
    //     }
    //     ans
    // }

    // fn preorder_dfs_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut ans = vec![];
    //
    //     if root.is_none() {
    //         return ans;
    //     }
    //     let mut stack = vec![root];
    //     while !stack.is_empty() {
    //         let node = stack.pop().flatten().unwrap();
    //         let node = node.borrow();
    //         ans.push(node.val);
    //         if let Some(ref right) = node.right {
    //             stack.push(Some(right.clone()));
    //         }
    //         if let Some(ref left) = node.left {
    //             stack.push(Some(left.clone()));
    //         }
    //     }
    //     ans
    // }
    //
    // fn preorder_dfs_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut ans = vec![];
    //
    //     if root.is_none() {
    //         return ans;
    //     }
    //     let mut stack = vec![root.unwrap()];
    //     while !stack.is_empty() {
    //         if let Some(node) = stack.pop() {
    //             ans.push(node.borrow().val);
    //             if let Some(ref right) = node.borrow().right {
    //                 stack.push(right.clone());
    //             }
    //             if
    //             let Some(ref left) = node.borrow().left {
    //                 stack.push(left.clone());
    //             }
    //         }
    //     }
    //     ans
    // }


    fn preorder_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(node) = stack.pop().flatten() {
                ans.push(node.borrow().val);
                stack.push(node.borrow().right.clone());
                stack.push(node.borrow().left.clone());
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
            (vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], vec![3, 9, 20, 15, 7]),
            (vec![], vec![]),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::preorder_traversal(root);
            assert_eq!(ans, answer)
        }
    }
}
