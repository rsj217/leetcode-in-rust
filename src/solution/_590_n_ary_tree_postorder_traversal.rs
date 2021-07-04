use std::rc::Rc;
use std::cell::RefCell;
use crate::datastruct::n_treenode::NTreeNode as TreeNode;

pub struct Solution {}

impl Solution {
    pub fn preorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::dfs(root)
    }

    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        fn _dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(x) = node {
                for i in &x.borrow().children {
                    _dfs(i, ans);
                }
                ans.push(x.borrow().val);
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
            (vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)], vec![5, 6, 3, 2, 4, 1]),
            (vec![Some(1), None, Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), None, Some(8), None, Some(9), Some(10), None, None, Some(11), None, Some(12), None, Some(13), None,
                  None, Some(14)], vec![2, 6, 14, 11, 7, 3, 12, 8, 4, 13, 9, 10, 5, 1], ),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::preorder(root);
            assert_eq!(ans, answer);
        }
    }
}
