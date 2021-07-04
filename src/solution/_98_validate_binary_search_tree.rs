use crate::datastruct::bin_treenode::TreeNode;
use rand::Rng;

/// 验证二叉搜索树
///
/// ### Problem
///
/// 给定一个二叉树，判断其是否是一个有效的二叉搜索树。
///
/// 假设一个二叉搜索树具有如下特征：
///
/// * 节点的左子树只包含小于当前节点的数。
/// * 节点的右子树只包含大于当前节点的数。
/// * 所有左子树和右子树自身必须也是二叉搜索树。
///
/// ```text
/// 示例 1:
/// 输入:
///   2
///  / \\
/// 1   3
/// 输出: true
///
/// 示例 2:
/// 输入:
///   5
///  / \\
/// 1   4
///    / \\
///   3   6
/// 输出: false
/// 解释: 输入为: [5,1,4,null,null,3,6]。
///     根节点的值为 5 ，但是其右子节点值为 4 。
/// ```
///
/// ### Tips
///
/// Note: rust 中可以使用 `i32::MIN`, `i32::MAX` 获取该类型的最大值和最小值。
/// 对于边界情况，需要处理溢出。因此在比较 边界的时候，传递的是 i64，将 i32 转换成 i64 进行比较。
///
/// 方法一 中序遍历:
///
/// 使用中序遍历的顺序是树节点的投影性质。存储一个 `prev` 节点，每次访问节点的时候比较 `prev < curnode` 。
/// 如果不是单调递增，则不符合二叉搜索树的定义。即可返回结果
///
/// 方法二 DFS递归：
///
/// 根据二叉树搜索树的定义，`左边的节点 < 当前节点 < 右边的节点` 左边的节点可能是左孩子，也可能是左孩子的右孩子
///
/// ```text
///      10
///     /
///    5
///     \\
///      8
/// ```
///
/// 因此每次递归的时候，需要传入一个下界（左边最大值）和一个上界（右边最小值）。使用 DFS 递归遍历即可。伪代码如下：
///
/// ```text
///     def dfs(node, lval, rval) -> bool:
///         # 空树属于二叉搜索树
///         if node is None:
///             return True
///
///         # 当前节点在 (lval, rval) 区间内，递归左右子树
///         if lval < node.val < rval:
///             # 递归左子树，当前节点为左子树的上界。
///             lvalid =  dfs(node.left, lval, node.val)
///
///             # 递归右子树，当前节点为右子树的下界。
///             rvalid =  dfs(node.right, node.val, rval)
///
///             # 左右子树都是二叉搜索树
///             return lvalid and rvalid
///
///         else: # 当前节点不在 (lval, rval) 区间内，不符合二叉搜索树定义，返回 False
///             return False
/// ```
///
/// dfs 方法中，通常可以使用 `and` 或 `or` 逻辑表达式通过短路提前返回，优化性能 `return dfs(node.left) and dfs(node.right)`

pub struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    /// 题目入口
    /// 方法一中序遍历：`Solution::inorder(root)`
    ///
    /// 方法二DFS递归遍历：`Solution::dfs(root)`
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let num = rand::thread_rng().gen_range(0..2);
        match num {
            0 => Solution::inorder(root),
            1 => Solution::inorder(root),
            _ => panic!("nums err"),
        }
    }

    /// dfs 递归算法，递归传如 上下界 `(lval, rval)` , 判定当前节点的 `val` 是否在上下界区间。
    /// 需要注意，`lval` 和 `rval` 是 `i64`，`val`是 `i32`，比较的时候，需要将 `i32` 转换成 `i64`
    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _dfs(node: &Option<Rc<RefCell<TreeNode>>>, lval: i64, rval: i64) -> bool {
            match node {
                None => true,
                Some(x) => {
                    let val = x.borrow().val as i64;
                    if lval < val && val < rval {
                        _dfs(&x.borrow().left, lval, val) && _dfs(&x.borrow().right, val, rval)
                    } else {
                        false
                    }
                }
            }
        }
        _dfs(&node, i64::MIN, i64::MAX)
    }

    /// 使用中序遍历方式。根据中序遍历的结果是树节点投影的属性。
    /// 可以有 `inorder[i-1] < inorder[i] < inorder[i+1]`
    /// 存储一个 prev_val 的值，中序遍历过程中，依次比较 `prev_val` 和 `cur_val`
    /// 需要注意 i32 的边界陷阱。
    pub fn inorder(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut node = node;
        let mut stack = vec![];
        let mut prev_val = i64::MIN;

        loop {
            while let Some(x) = node {
                stack.push(x.clone());
                node = x.borrow().left.clone();
            }

            if stack.is_empty() {
                break;
            }
            let x = stack.pop().unwrap();
            if x.borrow().val as i64 <= prev_val {
                return false;
            }
            prev_val = x.borrow().val as i64;
            node = x.borrow().right.clone();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![Some(2), Some(1), Some(3)], true),
            (vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)], false),
            (vec![Some(1), Some(1)], false),
            (vec![Some(0), Some(-1)], true),
            (vec![Some(-2147483648)], true),
        ];
        for (nums, answer) in test_case {
            let root = TreeNode::create(nums);
            let ans = Solution::is_valid_bst(root);
            assert_eq!(ans, answer)
        }
    }
}
