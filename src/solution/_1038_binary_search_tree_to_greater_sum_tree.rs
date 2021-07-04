use crate::datastruct::bin_treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;


/// [Problem](https://leetcode-cn.com/problems/binary-search-tree-to-greater-sum-tree/)`
/// --------------------------------------------------------------------------------------
///
/// 给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树（Greater Sum Tree），使每个节点 node 的新值等于原树中大于或等于
/// node.val 的值之和。
///
/// 提醒一下，二叉搜索树满足下列约束条件：
///
///
/// 节点的左子树仅包含键 小于 节点键的节点。
/// 节点的右子树仅包含键 大于 节点键的节点。
/// 左右子树也必须是二叉搜索树。
///
///
/// > 注意：该题目与 538: https://leetcode-cn.com/problems/convert-bst-to-greater-tree/ 相同
///
/// ```text
/// 示例 1：
///
/// 输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
/// 输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
///
/// 示例 2：
///
/// 输入：root = [0,null,1]
/// 输出：[1,null,1]
///
/// 示例 3：
///
/// 输入：root = [1,0,2]
/// 输出：[3,3,2]
///
/// 示例 4：
///
/// 输入：root = [3,2,4,1]
/// 输出：[7,9,4,10]
/// ```
///
/// Tips
/// ------
///
///
/// Answer
/// ------
///



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

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(4), Some(1), Some(6), Some(0), Some(2), Some(5), Some(7), None, None, None, Some(3), None, None, None, Some(8)], vec![Some(30), Some(36), Some(21), Some(36), Some(35), Some(26), Some(15), None, None, None, Some(33), None, None, None, Some(8)]),
            (vec![Some(0), None, Some(1)], vec![Some(1), None, Some(1)]),
            (vec![Some(1), Some(0), Some(2)], vec![Some(3), Some(3), Some(2)]),
            (vec![Some(3), Some(2), Some(4), Some(1)], vec![Some(7), Some(9), Some(4), Some(10)]),
        ];

        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::bst_to_gst(root);
            assert_eq!(TreeNode::literal(ans), answer)
        }
    }
}
