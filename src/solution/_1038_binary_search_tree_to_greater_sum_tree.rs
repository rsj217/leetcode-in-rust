use crate::datastruct::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        let mut node = root.clone();
        let mut prev = Rc::new(RefCell::new(TreeNode::new(0)));

        loop {
            while let Some(x) = node {
                stack.push(x.clone());
                node = x.borrow().right.clone();
            }

            if stack.is_empty() {
                break;
            }

            let x = stack.pop().unwrap();
            x.borrow_mut().val += prev.borrow().val;
            // x.borrow_mut().val = x.borrow().val + prev.borrow().val;
            prev = x.clone();
            node = x.borrow().left.clone();
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datastruct::treenode::print_tree;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(4), Some(1), Some(6), Some(0), Some(2), Some(5), Some(7), None, None, None, Some(3), None, None, None, Some(8)], vec![1])
        ];

        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::bst_to_gst(root);
            let s = print_tree(ans);
            println!("{}", s);
            println!("{:?}", answer);

            // Todo
            // assert_eq!(ans, answer)
        }
    }
}
