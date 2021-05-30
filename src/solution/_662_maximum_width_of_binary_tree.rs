use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));


        while !queue.is_empty() {
            let q_size = queue.len();
            let (mut start, mut end) = (0, 0);
            for i in 0..q_size {
                if let (Some(x), seq) = queue.pop_front().unwrap() {
                    if i == 0 {
                        start = seq;
                    }
                    if i == q_size - 1 {
                        end = seq;
                    }
                    if let Some(ref x) = x.borrow().left{
                         queue.push_back((Some(x.clone()), 2 * seq + 1));
                    }
                    if let Some(ref x) = x.borrow().right{
                        queue.push_back((Some(x.clone()), 2 * seq + 2));
                    }
                    // if x.borrow().left.is_some() {
                    //     queue.push_back((x.borrow().left.clone(), 2 * seq + 1));
                    // }
                    //
                    // if x.borrow().right.is_some() {
                    //     queue.push_back((x.borrow().right.clone(), 2 * seq + 2));
                    // }
                }
            }
            ans = max(ans, end - start + 1);
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
            (vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)], 4),
            (vec![Some(1), Some(3),  None, Some(5), Some(3)], 2),
            (vec![Some(1), Some(3), Some(2), Some(5)], 2),
            (vec![Some(1), Some(3), Some(2), Some(5), None, None, Some(9), Some(6), None, None, Some(7)], 8),
        ];

        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::width_of_binary_tree(root);
            assert_eq!(ans, answer);
        }
    }
}
