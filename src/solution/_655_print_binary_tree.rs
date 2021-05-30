use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

pub struct Solution {}

impl Solution {
    fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    1 + max(dfs(&node.borrow().left), dfs(&node.borrow().right))
                }
            }
        }
        dfs(root)
    }


    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = Solution::get_height(&root);
        let width = (1 << height) - 1;
        let mut ans = vec![vec!["".to_string(); width as usize]; height as usize];

        fn dfs(ans: &mut Vec<Vec<String>>, node: &Option<Rc<RefCell<TreeNode>>>, deep: usize, lo: usize, hi: usize) {
            if let Some(node) = node {
                let mid = lo + (hi - lo) / 2;
                ans[deep][mid] = node.borrow().val.to_string();
                dfs(ans, &node.borrow().left, deep + 1, lo, mid);
                dfs(ans, &node.borrow().right, deep + 1, mid + 1, hi);
            }
        }
        dfs(&mut ans, &root, 0usize, 0usize, width as usize);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(1), Some(2)], vec![vec!["".to_string(), "1".to_string(), "".to_string()],
                                          vec!["2".to_string(), "".to_string(), "".to_string()]]),
            (vec![Some(1), Some(2), Some(3), None, Some(4)], vec![vec!["".to_string(), "".to_string(), "".to_string(), "1".to_string(), "".to_string(), "".to_string(), "".to_string()],
                                                                  vec!["".to_string(), "2".to_string(), "".to_string(), "".to_string(), "".to_string(), "3".to_string(), "".to_string()],
                                                                  vec!["".to_string(), "".to_string(), "4".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]]),
            (vec![Some(1), Some(2), Some(5), Some(3), None, None, None, Some(4)], vec![vec!["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "1".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                                                                                       vec!["".to_string(), "".to_string(), "".to_string(), "2".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "5".to_string(), "".to_string(), "".to_string(), "".to_string()],
                                                                                       vec!["".to_string(), "3".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                                                                                       vec!["4".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ]),
        ];

        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::print_tree(root);
            assert_eq!(ans, answer)
        }
    }
}
