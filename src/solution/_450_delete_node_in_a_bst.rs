use crate::datastruct::bin_treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::delete_node_dfs(root, key)
    }

    pub fn delete_node_dfs(node: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match node {
            None => None,
            Some(x) => {
                if key < x.borrow().val {
                    let lnode = Solution::delete_node_dfs(x.borrow().left.clone(), key);
                    x.borrow_mut().left = lnode;
                    Some(x)
                } else if x.borrow().val < key {
                    let rnode = Solution::delete_node_dfs(x.borrow().right.clone(), key);
                    x.borrow_mut().right = rnode;
                    Some(x)
                } else {
                    if x.borrow().left.is_some() && x.borrow().right.is_some() {
                        let successor = Solution::minimum_dfs(x.borrow().right.clone());
                        let successor = successor.unwrap();
                        let rnode = Solution::delete_node_dfs(x.borrow().right.clone(), successor.borrow().val);
                        successor.borrow_mut().right = rnode;
                        successor.borrow_mut().left = x.borrow().left.to_owned();
                        Some(successor)
                    } else if x.borrow().left.is_some() {
                        x.borrow().left.clone()
                    } else if x.borrow().right.is_some() {
                        x.borrow().right.clone()
                    } else {
                        None
                    }
                }
            }
        };
        root
    }

    pub fn minimum_dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                if x.borrow().left.is_none() {
                    return Some(x);
                }
                Solution::minimum_dfs(x.borrow().left.to_owned())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 3, vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7)]),
            (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 10, vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]),
        ];
        for (nums, key, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::delete_node(root, key);
            assert_eq!(TreeNode::literal(ans), answer);
        }
    }
}
