use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;


/// 二分搜索树节点&结构定义, 搜索(search), 插入(insert), 删除(delete) 算法
///
/// 二分搜索树定义为每个节点都是可以比较大小，并且`左子树 < 根 < 右子树`。
/// 二分搜索树的节点和普通树节点稍有不同。普通树节点的数据域可以是任何类型。
/// 二分搜索的数据域实际上是一对`(key,val)`的元组。
/// 其中`key`用来比较排序，`val`才是实际的数据域需要存储的值。
/// 因此二分搜索树可以用来实现类似**字典**这样的结构，所以`key`不能是重复元素。
/// 由于二分搜索树的算法的 查找，插入，删除 都与`key`有关，为了阐述算法，下面的实现忽略了数据域`val`。
///
/// 树节点（BSTreeNode）的定义。三个字段
/// * 键值 `key`: i32类型 用于比较
/// * 左子树 `left`：Option<Rc<RefCell<BSTreeNode>>> 类型，递归定义
/// * 右子树 `right`: Option<Rc<RefCell<BSTreeNode>>>
#[derive(Debug, PartialEq, Eq)]
pub struct BSTreeNode {
    pub key: i32,
    pub left: Option<Rc<RefCell<BSTreeNode>>>,
    pub right: Option<Rc<RefCell<BSTreeNode>>>,
}

impl BSTreeNode {
    /// `new`方法为`BSTreeNode`的构造方法。
    /// 入参是用于比较的`key`。返回`BSTreeNode` 实例
    /// ### Example
    ///
    /// ```rust
    /// use leetcode_in_rust::datastruct::bs_treenode::BSTreeNode;
    /// let root = BSTreeNode::new(0);
    /// assert_eq!(root.key, 0);
    /// ```
    #[inline]
    pub fn new(key: i32) -> Self {
        BSTreeNode {
            key,
            left: None,
            right: None,
        }
    }

    /// `search_dfs` 是二分搜索树的查找算法。
    /// 即以`node`为根的树种查找`key`的节点，如果查找不到放回`None`。
    /// 使用了 DFS 方式递归查找，实现原理是二分搜索算法。
    /// `key` 比 `node` 节点的值小，递归查询左子树`node.left`，`key` 比 `node`节点的值大，递归查询右子树`node.right`
    /// 一旦命中目标，随即返回
    ///
    /// ### Example
    /// ```rust
    /// use leetcode_in_rust::datastruct::bs_treenode::BSTreeNode;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    /// let root = BSTreeNode::new(0);
    /// let root = Some(Rc::new(RefCell::new(root)));
    /// let ans = BSTreeNode::search_dfs(root.clone(), 0);
    /// assert_eq!(ans.unwrap().borrow().key, 0);
    /// let ans = BSTreeNode::search_dfs(root.clone(), 1);
    /// assert!(ans.is_none());
    /// ```
    pub fn search_dfs(node: Option<Rc<RefCell<BSTreeNode>>>, key: i32) -> Option<Rc<RefCell<BSTreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                if key < x.borrow().key {
                    BSTreeNode::search_dfs(x.borrow().left.clone(), key)
                } else if x.borrow().key < key {
                    BSTreeNode::search_dfs(x.borrow().right.clone(), key)
                } else { // x.borrow().key == key
                    Some(x)
                }
            }
        }
    }

    /// 二分搜索树节点高度算法，与二叉树的实现一致。`node.height = 1 + max(node.left.height + node.right.height)`
    pub fn get_height(node: Option<Rc<RefCell<BSTreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(x) => {
                1 + max(BSTreeNode::get_height(x.borrow().left.clone()),
                        BSTreeNode::get_height(x.borrow().right.clone()))
            }
        }
    }

    /// `insert_dfs` 是二分搜索树的插入方法。
    /// 以`node`为根的二分搜索树插入`key`。如果`key`已存在，则什么也不做（如果设计了val字段，此时可以更新val字段）.
    /// 该方法使用DFS递归实现，与`search_dfs`方法类似。比较`key`和当前`node.key`, 再分别递归左右子树进行插入操作。
    /// 该方法返回插入节点后的子树的根，因此返回上次递归调用的时候，上层的`node`节点需要拼接递归调用返回的子树的根节点。
    /// ```test
    ///
    /// fn insert(node, key):
    ///     if key < node.key:
    ///         node.key = insert(node.left, key)
    /// ```
    /// ### Example
    /// ```rust
    /// use leetcode_in_rust::datastruct::bs_treenode::{BSTreeNode, is_bst_valid};
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    /// let nums = vec![5, 2, 3, 7, 8, 9, 1];
    /// let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(5))));
    /// for i in nums {
    ///     root = BSTreeNode::insert_dfs(root, i);
    /// }
    /// assert!(is_bst_valid(&root));
    /// ```
    pub fn insert_dfs(node: Option<Rc<RefCell<BSTreeNode>>>, key: i32) -> Option<Rc<RefCell<BSTreeNode>>> {
        match node {
            // 当前节点为空，即所需要新增的节点，返回上次递归调用
            None => Some(Rc::new(RefCell::new(BSTreeNode::new(key)))),
            Some(x) => {
                // 比较当前node的key
                // 递归遍历左子树，拼接左子树插入返回的新的子树的根
                if key < x.borrow().key {
                    let lnode = BSTreeNode::insert_dfs(x.borrow().left.clone(), key);
                    x.borrow_mut().left = lnode;
                } else if x.borrow().key < key { // 递归遍历右子树，拼接右子树插入返回的新的子树的根
                    let rnode = BSTreeNode::insert_dfs(x.borrow().right.clone(), key);
                    x.borrow_mut().right = rnode;
                }
                // 当前节点作为新的子树的根返回更上层的递归调用
                Some(x)
            }
        }
    }

    /// `minimum_dfs`方法用于查找二分搜索树的最小值。
    /// 即以`node`为根的二分搜索树，查找其最小值。
    /// 因为二分搜索中序遍历是从小到大，最小值即为最左边的节点(未必是叶子节点)
    /// 使用`DFS`递归求解最小值，该方法可以辅助删除二分搜索树节点算法。
    pub fn minimum_dfs(node: Option<Rc<RefCell<BSTreeNode>>>) -> Option<Rc<RefCell<BSTreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                if x.borrow().left.is_none() {
                    return Some(x);
                }
                BSTreeNode::minimum_dfs(x.borrow().left.to_owned())
            }
        }
    }

    /// `delete_dfs` 方法是删除二叉树指定的节点。
    /// 即以`node`为根的二分搜索树删除`key`节点。
    /// 该方法同样适用`DFS`递归实现。
    /// 被删除的`key`节点有三种情况：
    ///
    /// * 叶子节点：直接返回`None`给上层调用即可。
    /// * 只有一个子树的节点：节点只有一个子树，当前节点从树摘除之后，返回其有值的子树即可。
    /// * 左右子树都存在的节点：需要从当前节点的右子树为根的子树中，找到最小值。然后把最小值和当前节点替换（此时依然符合二分搜索的性质）。然后再删除替换后右子树的`key`，即转换成第二种问题。
    ///
    /// 由于递归调用传入的`node`是通过`RC::clone`传入的，因此`node`只增加了引用计数，没有拷贝堆内存。在递归函数中，返回了新的树根。
    /// 那么传入的`node`随着函数调用完毕离开作用域，会自动减少引用计数。
    /// 当上层函数拼接子函数调用返回的树根的时候，因为重新赋值。`node`会被`rust`的所有权机制自动`drop`。不需要手动释放删除节点的内存。
    ///
    /// ### Example
    ///
    /// ```rust
    /// use leetcode_in_rust::datastruct::bs_treenode::{BSTreeNode, is_bst_valid};
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    ///
    /// let nums = vec![2, 1, 3];
    /// let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(4))));
    /// for i in nums {
    ///     root = BSTreeNode::insert_dfs(root, i)
    /// }
    ///
    /// root = BSTreeNode::delete_dfs(root, 4);
    /// assert!(is_bst_valid(&root));
    /// ```
    pub fn delete_dfs(node: Option<Rc<RefCell<BSTreeNode>>>, key: i32) -> Option<Rc<RefCell<BSTreeNode>>> {
        let root = match node {
            None => None,
            Some(x) => {
                if key < x.borrow().key {
                    let lnode = BSTreeNode::delete_dfs(x.borrow().left.clone(), key);
                    x.borrow_mut().left = lnode;
                    Some(x)
                } else if x.borrow().key < key {
                    let rnode = BSTreeNode::delete_dfs(x.borrow().right.clone(), key);
                    x.borrow_mut().right = rnode;
                    Some(x)
                } else {
                    if x.borrow().left.is_some() && x.borrow().right.is_some() {
                        let successor = BSTreeNode::minimum_dfs(x.borrow().right.clone());
                        let successor = successor.unwrap();
                        let rnode = BSTreeNode::delete_dfs(x.borrow().right.clone(), successor.borrow().key);
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
}

pub fn is_bst_valid(node: &Option<Rc<RefCell<BSTreeNode>>>) -> bool {
    let mut nums: Vec<i32> = vec![];

    fn order_dfs(node: &Option<Rc<RefCell<BSTreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(x) = node {
            order_dfs(&x.borrow().left, nums);
            nums.push(x.borrow().key);
            order_dfs(&x.borrow().right, nums);
        }
    }
    order_dfs(node, &mut nums);
    crate::algo::sort::helper::is_sorted(nums)
}

fn print_tree(root: Option<Rc<RefCell<BSTreeNode>>>) -> String {
    let height = BSTreeNode::get_height(root.clone());
    let width = (1 << height) - 1;
    let mut ans = vec![vec![" ".to_string(); width as usize]; height as usize];

    fn dfs(ans: &mut Vec<Vec<String>>, node: &Option<Rc<RefCell<BSTreeNode>>>, deep: usize, lo: usize, hi: usize) {
        if let Some(x) = node {
            let node = x.borrow();
            let mid = lo + (hi - lo) / 2;
            ans[deep][mid] = x.borrow().key.to_string();
            dfs(ans, &node.left, deep + 1, lo, mid);
            dfs(ans, &node.right, deep + 1, mid + 1, hi);
        }
    }
    dfs(&mut ans, &root.clone(), 0usize, 0usize, width as usize);
    ans.iter().map(|x| x.concat()).collect::<Vec<_>>().join("\n")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        println!("{}", "=".repeat(20));

        let nums = vec![5, 2, 3, 7, 8, 9, 1];
        let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(5))));
        for i in nums {
            root = BSTreeNode::insert_dfs(root, i);
        }
        assert!(is_bst_valid(&root));
        println!("{}", print_tree(root));
        println!("{}", "=".repeat(20));
    }

    #[test]
    fn test_search() {
        let nums = vec![5, 2, 3, 7, 8, 9, 1];
        let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(5))));
        for i in nums.clone() {
            root = BSTreeNode::insert_dfs(root, i);
        }

        for i in nums {
            let node5 = BSTreeNode::search_dfs(root.clone(), i);
            assert_eq!(node5.unwrap().borrow().key, i);
        }
        for i in 10..20 {
            assert!(BSTreeNode::search_dfs(root.clone(), i).is_none());
        }
    }

    #[test]
    fn test_delete_leaf() {
        println!("{}", "=".repeat(20));

        println!("test_delete_leaf");
        let nums = vec![2, 1, 3];
        let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(4))));
        for i in nums {
            root = BSTreeNode::insert_dfs(root, i)
        }
        println!("{}", print_tree(root.clone()));

        println!("delete 1");
        root = BSTreeNode::delete_dfs(root, 1);
        assert!(is_bst_valid(&root));
        println!("{}", print_tree(root));
        println!("{}", "=".repeat(20));
    }

    #[test]
    fn test_delete_left_child() {
        println!("{}", "=".repeat(20));

        println!("test_delete_left_child");
        let nums = vec![2, 1, 3];
        let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(4))));
        for i in nums {
            root = BSTreeNode::insert_dfs(root, i)
        }
        println!("{}", print_tree(root.clone()));

        println!("delete 4");
        root = BSTreeNode::delete_dfs(root, 4);
        assert!(is_bst_valid(&root));
        println!("{}", print_tree(root));
        println!("{}", "=".repeat(20));
    }

    #[test]
    fn test_delete_right_child() {
        println!("{}", "=".repeat(20));

        println!("test_delete_right_child");
        let nums = vec![2, 3];
        let mut root = Some(Rc::new(RefCell::new(BSTreeNode::new(4))));
        for i in nums {
            root = BSTreeNode::insert_dfs(root, i)
        }
        println!("{}", print_tree(root.clone()));

        println!("delete 2");
        root = BSTreeNode::delete_dfs(root, 2);
        assert!(is_bst_valid(&root));
        println!("{}", print_tree(root));
        println!("{}", "=".repeat(20));
    }
}
