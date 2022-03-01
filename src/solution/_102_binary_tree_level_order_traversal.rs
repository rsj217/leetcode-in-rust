use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::option::Option::Some;

pub struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let size = queue.len();
            let mut level = vec![];
            for _ in 0..size {
                if let Some(x) = queue.pop_front().flatten() {
                    let node = x.borrow();
                    level.push(node.val);
                    queue.push_back(node.left.clone());
                    queue.push_back(node.right.clone());
                }
            }
            if !level.is_empty() {
                ans.push(level);
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
            (vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], vec![
                vec![3],
                vec![9, 20],
                vec![15, 7]
            ]),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::level_order(root);
            assert_eq!(ans, answer);
        }
    }
}
