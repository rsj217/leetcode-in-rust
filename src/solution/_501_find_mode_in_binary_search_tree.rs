use crate::datastruct::bin_treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_counts = 1;
        let mut cur_counts = 1;
        let mut ans = vec![];
        let mut prev_val = None;

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

            let x = stack.pop().unwrap();
            match prev_val {
                Some(val) if val == x.borrow().val => cur_counts += 1,
                _ => cur_counts = 1,
            }

            if max_counts < cur_counts {
                ans = vec![prev_val.unwrap()];
                max_counts = cur_counts;
            } else if max_counts == cur_counts {
                ans.push(x.borrow().val)
            }
            prev_val = Some(x.borrow().val);
            node = x.borrow().right.clone();
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
            (vec![Some(1), None, Some(2), Some(2)], vec![2]),
            (vec![Some(6), Some(5), Some(8), Some(1), Some(5), None, None, Some(1)], vec![1, 5]),
            (vec![], vec![]),
            (vec![Some(1)], vec![1]),
            (vec![Some(0), None, Some(0)], vec![0]),
        ];

        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::find_mode(root);
            assert_eq!(ans, answer);
        }
    }
}
