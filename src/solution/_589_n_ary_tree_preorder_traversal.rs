use std::rc::Rc;
use std::cell::RefCell;
use crate::datastruct::n_treenode::NTreeNode as TreeNode;
use rand::{Rng, thread_rng};

pub struct Solution {}

impl Solution {
    pub fn preorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..2);
        match x {
            0 => Solution::dfs(root),
            1 => Solution::preorder_traversal(root),
            _ => vec![],
        }
    }

    pub fn preorder_traversal(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if node.is_none() {
            return ans;
        }
        let mut stack = vec![node];
        while !stack.is_empty() {
            if let Some(x) = stack.pop().flatten() {
                ans.push(x.borrow().val);
                let size = x.borrow().children.len();
                for i in (0..size).rev() {
                    stack.push(x.borrow().children[i].clone())
                }
            }
        }
        ans
    }

    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        fn _dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(x) = node {
                ans.push(x.borrow().val);
                for i in &x.borrow().children {
                    _dfs(i, ans);
                }
            }
        }
        _dfs(&node, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_literal() {
        let test_case = vec![
            (vec![], vec![]),
            (vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)], vec![1, 3, 5, 6, 2, 4]),
            (vec![Some(1), None, Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), None, Some(8), None, Some(9), Some(10), None, None, Some(11), None, Some(12), None, Some(13), None,
                  None, Some(14)], vec![1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10], ),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::preorder(root);
            assert_eq!(ans, answer);
        }
    }
}
