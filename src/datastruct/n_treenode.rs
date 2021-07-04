use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;


/// N叉树树节点&结构定义
///
#[derive(Debug, PartialEq, Eq)]
pub struct NTreeNode {
    pub val: i32,
    pub children: Vec<Option<Rc<RefCell<NTreeNode>>>>,
}

impl NTreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        NTreeNode {
            val,
            children: vec![],
        }
    }

    pub fn create(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<NTreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let root = Some(Rc::new(RefCell::new(NTreeNode::new(nums[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut parent = root.clone();
        for i in 1..nums.len() {
            if nums[i].is_none() {
                parent = queue.pop_front().flatten();
            } else {
                let node = Some(Rc::new(RefCell::new(NTreeNode::new(nums[i].unwrap()))));
                parent.as_ref().unwrap().borrow_mut().children.push(node.clone());
                queue.push_back(node.clone())
            }
        }
        root
    }

    pub fn literal(node: Option<Rc<RefCell<NTreeNode>>>) -> Vec<Option<i32>> {
        match node {
            None => vec![],
            Some(x) => {
                let mut queue = VecDeque::new();
                queue.push_back(Some(x.clone()));
                let mut ans = vec![Some(x.borrow().val), None];

                while !queue.is_empty() {
                    if let Some(x) = queue.pop_front().flatten() {
                        for i in &x.borrow().children {
                            ans.push(Some(i.as_ref().unwrap().borrow().val));
                            queue.push_back(i.clone());
                        }
                        ans.push(None);
                    }
                }
                let size = ans.len();
                for i in (0..size).rev() {
                    if ans[i].is_none() {
                        ans.pop();
                    } else {
                        break;
                    }
                }
                ans
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_literal() {
        let test_case = vec![
            vec![],
            vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)],
            vec![Some(1), None, Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), None, Some(8), None, Some(9), Some(10), None, None, Some(11), None, Some(12), None, Some(13), None,
                 None, Some(14)],
        ];
        for nums in test_case {
            let root = NTreeNode::create(nums.clone());
            assert_eq!(NTreeNode::literal(root), nums);
        }
    }
}
