use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::cmp::max;

/// 树节点&树结构定义，序列化/反序列化，打印，遍历相关算法
///
/// 树节点（TreeNode）的定义。三个字段
/// * 数据域 `val`: i32类型
/// * 左子树 `left`：Option<Rc<RefCell<TreeNode>>> 类型，递归定义
/// * 右子树 `right`: Option<Rc<RefCell<TreeNode>>>
///
/// 树节点相关的算法中，入参是`Option<Rc<RefCell<TreeNode>>>` 类型，直接传递会发生所有权转移。
/// 使用`clone`方法，调用的是`Rc`的`clone`方法，该方法只会增加引用计数，不会对堆内存进行深拷贝。
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// `new` 构造方法，用于初始化 `TreeNode` 实例
    ///
    /// # Example
    /// ```
    /// use leetcode_in_rust::datastruct::treenode::TreeNode;
    ///
    /// let root = TreeNode::new(1);
    /// ```
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// `get_height` 获取以节点`node`为`root`的树的树高。
    ///
    /// 定义空树`empty tree`的 `height` 为`0`，单个节点的`height`为`1`
    /// 该方法使用`DFS`递归方法实现，`node.height = 1 + max(node.left.height + node.right.height)`
    /// 入参是`Option<Rc<RefCell<TreeNode>>>`，内部的`dfs`遍历使用是其引用
    ///
    /// # Example
    ///
    /// ```
    /// use leetcode_in_rust::datastruct::treenode::TreeNode;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let node = TreeNode::new(1);
    /// let height = TreeNode::get_height(Some(Rc::new(RefCell::new(node))));
    /// ```
    pub fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    1 + max(dfs(&node.left), dfs(&node.right))  // 最后一行返回的表达式不需要分号 `;`
                }
            }
        }
        dfs(&root)
    }

    /// `create` 方法用于将的`Option<i32>`枚举的向量列表构造成一颗树，`Option`的值即为`TreeNode`的`val`即树的反序列化
    ///
    /// 向量的顺序是树的`BFS`层序遍历的顺序
    /// 除了最深的一层的叶子节点之外，其他的叶子节点的左右子树，使用`None`站位。位于最后一个节点之后的`None`则需要去掉
    /// 该方法与`literal`逆方法。后者将一棵树序列化成`Option<i32>`向量列表。
    /// 实现原理借助了完全二叉树层序遍历的序号(seq) 与 数组中索引(index)的关系。
    /// 由于有的None节点在数组中不存在，因此这个 seq 与 完全二叉树的节点 seq 有差别。但左右子树的序号与当前节点的序号关系依然成立
    ///
    /// # Example
    /// ```
    /// use leetcode_in_rust::datastruct::treenode::TreeNode;
    ///
    /// let nums = vec![
    ///     Some(1),
    ///     Some(2), Some(3),
    ///     Some(4), None, Some(5), Some(6),
    ///     None, Some(7), None, None, Some(8)
    /// ];
    /// let root = TreeNode::create(nums);
    ///
    /// ```
    /// root 的拓扑形状如下，可以使用该模块的 `print_tree` 打印树的拓扑
    /// ```text
    ///        1
    ///    2       3
    ///  4       5   6
    ///   7         8
    /// ```
    ///
    /// 叶子节点 2 的右子树，4的左子树 5 的左右子树都用 None 占位，叶子节点 6 的右子树在最后一个节点 8 之后，就不需要 None 占位了。
    ///
    /// 树的序列化和反序列化可以参考 Leetcode [297.二叉树序列化](https://leetcode-cn.com/problems/serialize-and-deserialize-binary-tree/)
    pub fn create(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if nums.is_empty() {
            return None;
        }
        let size = nums.len();
        // 初始化节点在数组中的索引
        let mut index = 0;
        // 初始化根(root)节点
        let root = Some(Rc::new(RefCell::new(Self::new(nums[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let qsize = queue.len();
            for _i in 0..qsize {
                if let Some(x) = queue.pop_front().flatten() {
                    let mut node = x.borrow_mut();

                    // 借用完全二叉树的性质
                    // `leftnode.seq = 2 * curnode.seq + 1`
                    // `rightnode.seq = 2 * curnode.seq + 2`
                    let lseq = 2 * index + 1;
                    let rseq = 2 * index + 2;
                    // 构造左子树
                    if lseq < size && nums[lseq].is_some() {
                        node.left = Some(Rc::new(RefCell::new(Self::new(nums[lseq].unwrap()))));
                        queue.push_back(node.left.clone());
                    }
                    // 构造右子树
                    if rseq < size && nums[rseq].is_some() {
                        node.right = Some(Rc::new(RefCell::new(Self::new(nums[rseq].unwrap()))));
                        queue.push_back(node.right.clone());
                    }
                }
                // 增加序号
                index += 1;
            }
        }
        root
    }

    /// `literal` 是将一颗二叉树序列化成`Option<i32>`向量列表。是`create`方法的逆方法。
    ///
    /// 其原理是使用二叉树的`BFS`层序遍历依次解析节点。
    /// 从树根开始遍历，如果当前节点不存在，直接放入输出`None`进行占位。如果节点存在，则将其值输出到结果。
    /// 遍历完成之后，再剔除最后一个节点之后的`None`值。
    ///
    /// # Example
    ///
    /// ```
    /// use leetcode_in_rust::datastruct::treenode::TreeNode;
    ///
    /// let nums = vec![
    ///     Some(1),Some(2), Some(3),Some(4), None, Some(5), Some(6),None, Some(7), None, None, Some(8),
    /// ];
    /// let root = TreeNode::create(nums.clone());
    /// let ans = TreeNode::literal(root);
    /// assert_eq!(nums, ans);
    /// ```
    pub fn literal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let qsize = queue.len();
            for _ in 0..qsize {
                match queue.pop_front().flatten() {
                    // 当前节点有值，输出结果值，左右子树进队
                    Some(x) => {
                        ans.push(Some(x.borrow().val));
                        queue.push_back(x.borrow().left.clone());
                        queue.push_back(x.borrow().right.clone());
                    }
                    // 当前结果None，输出None
                    None => ans.push(None),
                }
            }
        }
        // 剔除最后一个节点之后的 None 值
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

/// `print_tree` 函数用于打印二叉树的树形拓扑，方便直观的观察一棵树的形状。便于验证相关树算法。
///
/// 拓扑结果中，并没有画出路径，由于控制台输出长度的限制，树的节点数也有限制。树的宽度 `width = (1<<height) - 1`，不能太大。
/// 函数使用了`DFS`方式遍历一棵树。
///
/// # Example
/// ```
/// use leetcode_in_rust::datastruct::treenode::{TreeNode, print_tree};
///
/// let nums = vec![Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, Some(7), None, None, Some(8)];
/// let root = TreeNode::create(nums);
/// println!("{}", print_tree(root));
/// ```
/// root 的拓扑形状如下
///        1
///    2       3
///  4       5   6
///   7         8
///
///  叶子节点 2 的右子树，4的左子树 5 的左右子树都用 None 占位，叶子节点 6 的右子树在最后一个节点 8 之后，就不需要 None 占位了。
pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let height = TreeNode::get_height(root.clone());
    let width = (1 << height) - 1;
    let mut ans = vec![vec![" ".to_string(); width as usize]; height as usize];

    fn dfs(ans: &mut Vec<Vec<String>>, node: &Option<Rc<RefCell<TreeNode>>>, deep: usize, lo: usize, hi: usize) {
        if let Some(x) = node {
            let node = x.borrow();
            let mid = lo + (hi - lo) / 2;
            ans[deep][mid] = x.borrow().val.to_string();
            dfs(ans, &node.left, deep + 1, lo, mid);
            dfs(ans, &node.right, deep + 1, mid + 1, hi);
        }
    }
    dfs(&mut ans, &root.clone(), 0usize, 0usize, width as usize);
    ans.iter().map(|x| x.concat()).collect::<Vec<_>>().join("\n")
}

