use std::rc::Rc;
use std::cell::RefCell;
use crate::datastruct::treenode::TreeNode;

/// [Problem](https://leetcode-cn.com/problems/insert-into-a-binary-search-tree/)
/// ---------------------------------------------------------------------------------
///
/// 给定二叉搜索树（BST）的根节点和要插入树中的值，将值插入二叉搜索树。 返回插入后二叉搜索树的根节点。 输入数据 保证 ，新值和原始二叉搜索树中的任意节点值
/// 都不同。
///
/// 注意，可能存在多种有效的插入方式，只要树在插入后仍保持为二叉搜索树即可。 你可以返回 任意有效的结果 。
///
/// ```text
///
/// 示例 1：
///
/// 输入：root = [4,2,7,1,3], val = 5
/// 输出：[4,2,7,1,3,5]
/// 解释：另一个满足题目要求可以通过的树是：
///
/// 示例 2：
///
/// 输入：root = [40,20,60,10,30,50,70], val = 25
/// 输出：[40,20,60,10,30,50,70,null,null,25]
///
///
/// 示例 3：
///
/// 输入：root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
/// 输出：[4,2,7,1,3,5]
///
/// 提示：
///
/// 给定的树上的节点数介于 0 和 10^4 之间
/// 每个节点都有一个唯一整数值，取值范围从 0 到 10^8
/// -10^8 <= val <= 10^8
/// 新值和原始二叉搜索树中的任意节点值都不同
///
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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::insert_dfs(root, val)
    }

    pub fn insert_dfs(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(x) => {
                if val < x.borrow().val {
                    let lnode = Solution::insert_dfs(x.borrow().left.clone(), val);
                    x.borrow_mut().left = lnode;
                } else if x.borrow().val < val {
                    let rnode = Solution::insert_dfs(x.borrow().right.clone(), val);
                    x.borrow_mut().right = rnode;
                }
                Some(x)
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
            (vec![Some(4), Some(2), Some(7), Some(1), Some(3)], 5, vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]),
            (vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70)], 25, vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), None, None, Some(25)]),
        ];
        for (nums, val, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::insert_into_bst(root, val);
            assert_eq!(TreeNode::literal(ans), answer);
        }
    }
}
