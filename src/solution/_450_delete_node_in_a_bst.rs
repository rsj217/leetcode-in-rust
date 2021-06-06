use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::delete_node_dfs(root, key)
    }

    pub fn delete_node_dfs(node: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() {
            return node;
        }

        let x = node.unwrap();
        if key < x.borrow().val {
            let lnode = Solution::delete_node_dfs(x.borrow().left.clone(), key);
            x.borrow_mut().left = lnode;
            return Some(x.clone());
        } else if x.borrow().val < key {
            let rnode = Solution::delete_node_dfs(x.borrow().right.clone(), key);
            x.borrow_mut().right = rnode;
            return Some(x.clone());
        } else { // val == key
            if x.borrow().left.is_none() {
                return x.borrow().right.clone();
            }
            if x.borrow().right.is_none() {
                return x.borrow().left.clone();
            }
            // 当前节点的左右子树一定存在，返回的最小值 successor 也一定存在
            let successor = Solution::minimum(x.borrow().right.clone());
            let successor = successor.unwrap();
            let rnode = Solution::delete_node_dfs(x.borrow().right.clone(), successor.borrow().val);
            successor.borrow_mut().right = rnode;
            let lnode = x.borrow().left.clone();
            successor.borrow_mut().left = lnode;
            return Some(successor);
        }
    }

    pub fn minimum(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        assert!(node.is_some(), "node err");
        let x = node.as_ref().unwrap();
        if x.borrow().left.is_none() {
            return Some(x.clone());
        }
        Solution::minimum(x.borrow().left.clone())
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
