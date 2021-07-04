use std::rc::Rc;
use std::cell::RefCell;
use crate::datastruct::bin_treenode::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;
        while let Some(x) = node {
            if x.borrow().val < val {
                node = x.borrow().right.clone();
            } else if val < x.borrow().val {
                node = x.borrow().left.clone();
            } else {
                return Some(x);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(4), Some(2), Some(7), Some(1), Some(3)], 2, vec![Some(2), Some(1), Some(3)]),
            (vec![Some(2), Some(1), Some(3)], 5, vec![]),
        ];
        for (nums, val, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::search_bst(root, val);
            assert_eq!(TreeNode::literal(ans), answer);
        }
    }
}
