use std::cell::RefCell;
use std::rc::Rc;
use crate::datastruct::bin_treenode::TreeNode;

pub struct Solution {}


impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            match node {
                None => None,
                Some(x) => {
                    if x.borrow().val == p.as_ref().unwrap().borrow().val || x.borrow().val == q.as_ref().unwrap().borrow().val {
                        return Some(x.clone());
                    }

                    let left = dfs(&x.borrow().left, p, q);
                    let right = dfs(&x.borrow().right, p, q);
                    if left.is_some() && right.is_some() {
                        return Some(x.clone());
                    }
                    if left.is_none() {
                        return right;
                    }
                    if right.is_none() {
                        return left;
                    }
                    None
                }
            }
        }
        dfs(&root, &p, &q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)], vec![Some(5)], vec![Some(1)], 3),
            (vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)], vec![Some(5)], vec![Some(4)], 5),
            (vec![Some(1), Some(2)], vec![Some(1)], vec![Some(2)], 1),
        ];
        for (nums, p, q, answer) in test_case {
            let root = TreeNode::create(nums);
            let p = TreeNode::create(p);
            let q = TreeNode::create(q);
            let ans = Solution::lowest_common_ancestor(root, p, q);
            assert_eq!(ans.unwrap().borrow().val, answer)
        }
    }
}
