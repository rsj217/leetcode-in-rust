use crate::datastruct::bin_treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
            match node {
                None => 0,
                Some(x) => {
                    if low <= x.borrow().val && x.borrow().val <= high {
                        x.borrow().val + dfs(&x.borrow().left, low, high) + dfs(&x.borrow().right, low, high)
                    } else if x.borrow().val < low {
                        dfs(&x.borrow().right, low, high)
                    } else { //  high < x.borrow().val
                        dfs(&x.borrow().left, low, high)
                    }
                }
            }
        }
        dfs(&root, low, high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)], 7, 15, 32),
            (vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)], 6, 10, 23),
        ];

        for (nums, low, high, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::range_sum_bst(root, low, high);
            assert_eq!(ans, answer);
        }
    }
}
