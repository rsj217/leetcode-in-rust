use crate::datastruct::bin_treenode::TreeNode;

pub struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::postorder(root)
    }

    fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;
        let mut visited = None;

        loop {
            while let Some(x) = node {
                stack.push(x.clone());
                node = x.borrow().left.clone();
            }
            if stack.is_empty() {
                break;
            }
            if stack.last().unwrap().borrow().right != visited {
                node = stack.last().unwrap().borrow().right.clone();
                visited = None
            } else {
                if let Some(x) = stack.pop(){
                    ans.push(x.borrow().val);
                    visited = Some(x.clone());
                }
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
            (vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], vec![9, 15, 7,  20, 3]),
            (vec![Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, Some(7), None, None, Some(8)], vec![7, 4, 2, 5, 8, 6, 3, 1]),
            (vec![], vec![]),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::postorder(root);
            assert_eq!(ans, answer)
        }
    }
}
