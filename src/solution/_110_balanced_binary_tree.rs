use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            match node {
                None => (true, 0),
                Some(x) => {
                    let (lans, lheight) = dfs(&x.borrow().left);
                    let (rans, rheight) = dfs(&x.borrow().right);
                    let curheight = 1 + max(lheight, rheight);
                    let abs = lheight - rheight;
                    if abs < -1 || abs > 1 || !lans || !rans {
                        return (false, curheight);
                    }
                    (true, curheight)
                }
            }
        }
        let (ans, _) = dfs(&root);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![], true),
            (vec![Some(1)], true),
            (vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], true),
            (vec![Some(1), Some(2), None, Some(3)], false),
            (vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(7), Some(8)], false),
            (vec![Some(1), Some(2), None, Some(4), None, Some(5)], false),
            (vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)], false),
            (vec![Some(3), Some(2), None, Some(1)], false),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::is_balanced(root);
            assert_eq!(ans, answer);
        }
    }
}
