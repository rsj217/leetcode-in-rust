use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

/// 树节点&树结构定义，序列化/反序列化，打印，遍历相关算法
///
/// 树节点（AVLTreeNode）的定义。三个字段
/// * 数据域 `val`: i32类型
/// * 左子树 `left`：Option<Rc<RefCell<AVLTreeNode>>> 类型，递归定义
/// * 右子树 `right`: Option<Rc<RefCell<AVLTreeNode>>>
/// * 高度 `height`: i32类型
///
#[derive(Debug, PartialEq, Eq)]
pub struct AVLTreeNode {
    pub key: i32,
    pub height: i32,
    pub left: Option<Rc<RefCell<AVLTreeNode>>>,
    pub right: Option<Rc<RefCell<AVLTreeNode>>>,
}

impl AVLTreeNode {
    #[inline]
    pub fn new(key: i32) -> Self {
        AVLTreeNode {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }

    /// `get_height` 方法用于获取节点的高度，节点不存在的时候，返回0。
    ///
    /// ### Example
    /// ```rust
    /// use leetcode_in_rust::datastruct::avl_treenode::AVLTreeNode;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(0))));
    /// let height = AVLTreeNode::get_height(root);
    /// assert_eq!(height, 1);
    ///
    /// let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(0))));
    /// let height = AVLTreeNode::get_height(root.as_ref().unwrap().borrow().left.clone());
    /// assert_eq!(height, 0);
    /// ```
    pub fn get_height(node: Option<Rc<RefCell<AVLTreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(x) => x.borrow().height,
        }
    }

    /// `get_bf` 方法获取节点的*平衡因子*（balance factor）。当平衡因子的绝对值大于1的时候，该节点即不平衡。
    /// `AVL`树要求每一个节点都是平衡的。因此判定每个节点的平衡因子，就可以判定是否是平衡树。
    ///
    /// ### Example
    /// ```rust
    /// use leetcode_in_rust::datastruct::avl_treenode::AVLTreeNode;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(0))));
    /// let bf = AVLTreeNode::get_bf(root);
    /// assert_eq!(bf, 0);
    /// ```
    pub fn get_bf(node: Option<Rc<RefCell<AVLTreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(x) => {
                AVLTreeNode::get_height(x.borrow().left.clone()) - AVLTreeNode::get_height(x.borrow().right.clone())
            }
        }
    }

    pub fn is_balance(node: Option<Rc<RefCell<AVLTreeNode>>>) -> bool {
        match node {
            None => true,
            Some(x) => {
                let bf = AVLTreeNode::get_bf(Some(x.clone()));
                if bf < -1 || bf > 1 {
                    false
                } else {
                    AVLTreeNode::is_balance(x.borrow().left.clone()) && AVLTreeNode::is_balance(x.borrow().right.clone())
                }
            }
        }
    }

    /// TODO
    pub fn is_bst(node: Option<Rc<RefCell<AVLTreeNode>>>) -> bool {
        true
    }
    /// `update_height`:更新节点的高度方法，根据节点的左右子树计算高度，返回更新后的节点。
    pub fn update_height(node: Option<Rc<RefCell<AVLTreeNode>>>) -> Option<Rc<RefCell<AVLTreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                let height = 1 + max(AVLTreeNode::get_height(x.borrow().left.clone()),
                                     AVLTreeNode::get_height(x.borrow().right.clone()));
                x.borrow_mut().height = height;
                Some(x)
            }
        }
    }

    /// `left_rotate` 是左旋转方法，与`right_rotate`的对称方法。下图向右边情况，`x` 节点失衡，需要向左旋转，以达到各个节点平衡。
    /// 旋转之后，需要更新节点的高度，优先更新子节点高度，再更新父节点高度，最后返回旋转后的子树的根节点
    ///
    /// ```text
    ///    x                               y
    ///  /   \                           /   \
    /// T1    y     Left Rotate(x)      x     z
    ///      / \   - - - - - - - ->    / \   / \
    ///    T2   z                     T1 T2 T3 T4
    ///        / \
    ///      T3   T4
    ///
    /// ```
    ///
    pub fn left_rotate(node: Option<Rc<RefCell<AVLTreeNode>>>) -> Option<Rc<RefCell<AVLTreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                let y = x.borrow().right.clone().unwrap();
                let t2 = y.borrow().left.clone();
                x.borrow_mut().right = t2;
                y.borrow_mut().left = Some(x.clone());

                AVLTreeNode::update_height(Some(x));
                AVLTreeNode::update_height(Some(y))
            }
        }
    }

    /// `right_rotate` 是右旋转方法，与`left_rotate`的对称方法。下图向右边情况，`x` 节点失衡，需要向左旋转，以达到各个节点平衡。
    /// 旋转之后，需要更新节点的高度，优先更新子节点高度，再更新父节点高度，最后返回旋转后的子树的根节点
    ///
    /// ```text
    ///        x                                     y
    ///       / \                                  /   \
    ///      y   T4      Right Rotate (x)         z     x
    ///     / \          - - - - - - - - ->      / \   / \
    ///    z   T3                               T1 T2 T3  T4
    ///   / \
    /// T1   T2
    /// ```
    ///
    pub fn right_rotate(node: Option<Rc<RefCell<AVLTreeNode>>>) -> Option<Rc<RefCell<AVLTreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                let y = x.borrow().left.clone().unwrap();
                let t3 = y.borrow().right.clone();
                x.borrow_mut().left = t3;
                y.borrow_mut().right = Some(x.clone());

                AVLTreeNode::update_height(Some(x));
                AVLTreeNode::update_height(Some(y))
            }
        }
    }

    /// `balance_rotate` 是平衡调整方法。
    ///
    /// *LL*
    ///
    /// ```text
    ///        x                                     y
    ///       / \                                  /   \
    ///      y   T4      right rotate (x)         z     x
    ///     / \          - - - - - - - - ->      / \   / \
    ///    z   T3                               T1 T2 T3 T4
    ///   / \
    /// T1   T2
    /// ```
    /// *LR*
    ///
    /// ```text
    ///      x                               x                           y
    ///     / \                            /   \                       /   \
    ///    y   T4  Left Rotate (y)        y    T4  Right Rotate(x)    z     x
    ///   / \      - - - - - - - - ->    /  \      - - - - - - - ->  / \   / \
    ///  T1  z                          z    T3                     T1 T2 T3 T4
    ///     / \                        / \
    ///   T2   T3                    T1   T2
    /// ```
    ///
    /// *RR*
    ///
    /// ```text
    ///   x                               y
    ///  / \                            /   \
    /// T1  y     Left Rotate(x)       z     x
    ///    / \   - - - - - - - ->     / \   / \
    ///   T2  z                      T1 T2 T3 T4
    ///      / \
    ///     T3 T4
    /// ```
    /// *RL*
    ///
    /// ```text
    ///    x                            x                             z
    ///   / \                          / \                          /   \
    /// T1   y   Right Rotate (y)    T1   z      Left Rotate(x)    x     y
    ///     / \  - - - - - - - - ->     /  \   - - - - - - - ->   / \   / \
    ///    z   T4                      T2   y                    T1 T2 T3 T4
    ///   / \                              / \
    ///  T2 T3                            T3 T4
    /// ```
    pub fn balance_rotate(node: Option<Rc<RefCell<AVLTreeNode>>>) -> Option<Rc<RefCell<AVLTreeNode>>> {
        let bf = AVLTreeNode::get_bf(node.clone());

        if bf > 1 {
            // LL LR
            let x = node.as_ref().unwrap();
            if AVLTreeNode::get_bf(x.borrow().left.clone()) >= 0 {
                // LL
                AVLTreeNode::right_rotate(node)
            } else {
                // LR
                let lnode = AVLTreeNode::left_rotate(x.borrow().left.clone());
                x.borrow_mut().left = lnode;
                AVLTreeNode::right_rotate(node)
            }
        } else if bf < -1 {
            // RR RL
            let x = node.as_ref().unwrap();
            if AVLTreeNode::get_bf(x.borrow().right.clone()) <= 0 {
                // RR
                AVLTreeNode::left_rotate(node)
            } else {
                // RL
                let rnode = AVLTreeNode::right_rotate(x.borrow().right.clone());
                x.borrow_mut().right = rnode;
                AVLTreeNode::left_rotate(node)
            }
        } else {
            node
        }
    }

    /// `insert_dfs`: 以`node`为根的平衡树插入`key`节点
    ///
    pub fn insert_dfs(node: Option<Rc<RefCell<AVLTreeNode>>>, key: i32) -> Option<Rc<RefCell<AVLTreeNode>>> {
        let node = match node {
            None => Some(Rc::new(RefCell::new(AVLTreeNode::new(key)))),
            Some(x) => {
                if key < x.borrow().key {
                    let lnode = AVLTreeNode::insert_dfs(x.borrow().left.clone(), key);
                    x.borrow_mut().left = lnode;
                } else if x.borrow().key < key {
                    let rnode = AVLTreeNode::insert_dfs(x.borrow().right.clone(), key);
                    x.borrow_mut().right = rnode;
                }
                Some(x)
            }
        };
        let node = AVLTreeNode::update_height(node);
        AVLTreeNode::balance_rotate(node)
    }

    pub fn minimum_dfs(node: Option<Rc<RefCell<AVLTreeNode>>>) -> Option<Rc<RefCell<AVLTreeNode>>> {
        match node {
            None => None,
            Some(x) => {
                if x.borrow().left.is_none() {
                    Some(x)
                } else {
                    AVLTreeNode::minimum_dfs(x.borrow().left.clone())
                }
            }
        }
    }

    /// `delete_dfs`： 以`node`为根删除`key`的节点
    ///
    ///
    pub fn delete_dfs(node: Option<Rc<RefCell<AVLTreeNode>>>, key: i32) -> Option<Rc<RefCell<AVLTreeNode>>> {
        let node = match node {
            None => None,
            Some(x) => {
                if key < x.borrow().key {
                    let lnode = AVLTreeNode::delete_dfs(x.borrow().left.clone(), key);
                    x.borrow_mut().left = lnode;
                    Some(x)
                } else if x.borrow().key < key {
                    let rnode = AVLTreeNode::delete_dfs(x.borrow().right.clone(), key);
                    x.borrow_mut().right = rnode;
                    Some(x)
                } else {
                    if x.borrow().left.is_some() && x.borrow().right.is_some() {
                        let successor = AVLTreeNode::minimum_dfs(x.borrow().right.clone()).unwrap();
                        let rnode = AVLTreeNode::delete_dfs(x.borrow().right.clone(), successor.borrow().key);
                        successor.borrow_mut().right = rnode;
                        successor.borrow_mut().left = x.borrow().left.clone();
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
        AVLTreeNode::update_height(node.clone());
        AVLTreeNode::balance_rotate(node)
    }
}

fn print_tree(root: Option<Rc<RefCell<AVLTreeNode>>>) -> String {
    let height = AVLTreeNode::get_height(root.clone());
    let width = (1 << height) - 1;
    let mut ans = vec![vec![" ".to_string(); width as usize]; height as usize];

    fn dfs(ans: &mut Vec<Vec<String>>, node: &Option<Rc<RefCell<AVLTreeNode>>>, deep: usize, lo: usize, hi: usize) {
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
    fn test_left_rotate() {
        println!("test_left_rotate");
        let x = Rc::new(RefCell::new(AVLTreeNode::new(1)));
        let y = Rc::new(RefCell::new(AVLTreeNode::new(2)));
        let z = Rc::new(RefCell::new(AVLTreeNode::new(3)));

        y.borrow_mut().height = 2;
        y.borrow_mut().right = Some(z);

        x.borrow_mut().height = 3;
        x.borrow_mut().right = Some(y);

        let root = Some(x);
        let root = AVLTreeNode::left_rotate(root);
        let y = root.as_ref().unwrap().borrow();
        assert_eq!(y.height, 2);
        assert_eq!(y.key, 2);

        assert_eq!(y.left.as_ref().unwrap().borrow().key, 1);
        assert_eq!(y.right.as_ref().unwrap().borrow().key, 3);

        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
        println!("{}", print_tree(root.clone()));
    }


    #[test]
    fn test_right_rotate() {
        println!("test_right_rotate");
        let x = Rc::new(RefCell::new(AVLTreeNode::new(3)));
        let y = Rc::new(RefCell::new(AVLTreeNode::new(2)));
        let z = Rc::new(RefCell::new(AVLTreeNode::new(1)));

        y.borrow_mut().height = 2;
        y.borrow_mut().left = Some(z);

        x.borrow_mut().height = 3;
        x.borrow_mut().left = Some(y);

        let root = Some(x);
        let root = AVLTreeNode::right_rotate(root);
        let y = root.as_ref().unwrap().borrow();
        assert_eq!(y.height, 2);
        assert_eq!(y.key, 2);

        assert_eq!(y.left.as_ref().unwrap().borrow().key, 1);
        assert_eq!(y.right.as_ref().unwrap().borrow().key, 3);
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
        println!("{}", print_tree(root.clone()));
    }

    /// LL
    /// ```text
    ///        3                         2
    ///       /                         / \
    ///      2    right_rotate(3) ->   1   3
    ///     /
    ///    1
    /// ```
    #[test]
    fn test_balance_rotate_left_left() {
        println!("test_balance_rotate_left_left");
        let x = Rc::new(RefCell::new(AVLTreeNode::new(3)));
        let y = Rc::new(RefCell::new(AVLTreeNode::new(2)));
        let z = Rc::new(RefCell::new(AVLTreeNode::new(1)));

        y.borrow_mut().height = 2;
        y.borrow_mut().left = Some(z);

        x.borrow_mut().height = 3;
        x.borrow_mut().left = Some(y);

        let root = Some(x);
        let root = AVLTreeNode::balance_rotate(root);
        let y = root.as_ref().unwrap().borrow();
        assert_eq!(y.height, 2);
        assert_eq!(y.key, 2);
        assert_eq!(y.left.as_ref().unwrap().borrow().key, 1);
        assert_eq!(y.right.as_ref().unwrap().borrow().key, 3);
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
        println!("{}", print_tree(root.clone()));
    }

    /// LR
    /// ```text
    ///    3                           3                        2
    ///   /                           /                        / \
    ///  1    left_rotate(1)   ->    2   right_rotate(3) ->   1   3
    ///   \                         /
    ///    2                       1
    /// ```
    #[test]
    fn test_balance_rotate_left_right() {
        println!("test_balance_rotate_left_right");
        let x = Rc::new(RefCell::new(AVLTreeNode::new(3)));
        let y = Rc::new(RefCell::new(AVLTreeNode::new(1)));
        let z = Rc::new(RefCell::new(AVLTreeNode::new(2)));

        y.borrow_mut().height = 2;
        y.borrow_mut().right = Some(z);

        x.borrow_mut().height = 3;
        x.borrow_mut().left = Some(y);

        let root = Some(x);
        let root = AVLTreeNode::balance_rotate(root);
        let y = root.as_ref().unwrap().borrow();
        assert_eq!(y.height, 2);
        assert_eq!(y.key, 2);
        assert_eq!(y.left.as_ref().unwrap().borrow().key, 1);
        assert_eq!(y.right.as_ref().unwrap().borrow().key, 3);
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
        println!("{}", print_tree(root.clone()));
    }

    /// RR
    /// ```text
    ///     1                            2
    ///      \                          / \
    ///       2    left_rotate(1) ->   1   3
    ///        \
    ///         3
    /// ```
    #[test]
    fn test_balance_rotate_right_right() {
        println!("test_balance_rotate_right_right");
        let x = Rc::new(RefCell::new(AVLTreeNode::new(1)));
        let y = Rc::new(RefCell::new(AVLTreeNode::new(2)));
        let z = Rc::new(RefCell::new(AVLTreeNode::new(3)));

        y.borrow_mut().height = 2;
        y.borrow_mut().right = Some(z);

        x.borrow_mut().height = 3;
        x.borrow_mut().right = Some(y);

        let root = Some(x);
        let root = AVLTreeNode::balance_rotate(root);
        let y = root.as_ref().unwrap().borrow();
        assert_eq!(y.height, 2);
        assert_eq!(y.key, 2);
        assert_eq!(y.left.as_ref().unwrap().borrow().key, 1);
        assert_eq!(y.right.as_ref().unwrap().borrow().key, 3);
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
        println!("{}", print_tree(root.clone()));
    }

    /// RL
    /// ```text
    ///  1                      1                           2
    ///   \                      \                         / \
    ///    3  right_rotate(3) ->  2   left_rotate(1) ->   1   3
    ///   /                        \
    ///  2                          3
    /// ```
    #[test]
    fn test_balance_rotate_right_left() {
        println!("test_balance_rotate_right_left");
        let x = Rc::new(RefCell::new(AVLTreeNode::new(1)));
        let y = Rc::new(RefCell::new(AVLTreeNode::new(3)));
        let z = Rc::new(RefCell::new(AVLTreeNode::new(2)));

        y.borrow_mut().height = 2;
        y.borrow_mut().left = Some(z);

        x.borrow_mut().height = 3;
        x.borrow_mut().right = Some(y);

        let root = Some(x);
        let root = AVLTreeNode::balance_rotate(root);
        let y = root.as_ref().unwrap().borrow();
        assert_eq!(y.height, 2);
        assert_eq!(y.key, 2);
        assert_eq!(y.left.as_ref().unwrap().borrow().key, 1);
        assert_eq!(y.right.as_ref().unwrap().borrow().key, 3);
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
        println!("{}", print_tree(root.clone()));
    }

    #[test]
    fn test_insert_left_left() {
        println!("test_insert_left_left");

        let nums = vec![2, 1];
        let mut root = Some(Rc::new(RefCell::new(AVLTreeNode::new(3))));
        for i in nums {
            root = AVLTreeNode::insert_dfs(root, i);
        }
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    #[test]
    fn test_insert_left_right() {
        println!("test_insert_left_right");

        let nums = vec![1, 2];
        let mut root = Some(Rc::new(RefCell::new(AVLTreeNode::new(3))));
        for i in nums {
            root = AVLTreeNode::insert_dfs(root, i);
        }
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    #[test]
    fn test_insert_right_right() {
        println!("test_insert_right_right");

        let nums = vec![2, 3];
        let mut root = Some(Rc::new(RefCell::new(AVLTreeNode::new(1))));
        for i in nums {
            root = AVLTreeNode::insert_dfs(root, i);
        }
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    #[test]
    fn test_insert_right_left() {
        println!("test_insert_right_left");

        let nums = vec![1, 2];
        let mut root = Some(Rc::new(RefCell::new(AVLTreeNode::new(3))));
        for i in nums {
            root = AVLTreeNode::insert_dfs(root, i);
        }
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    /// ```text
    ///        3                     3                        2
    ///       / \                   /                        / \
    ///      2   4   delete(4) ->  2    right_rotate(3) ->  1   2
    ///     /                     /
    ///    1                     1
    /// ```
    #[test]
    fn test_delete_left_left(){
        let nums = vec![2, 4, 1];
        let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(3))));
        for i in nums{
            AVLTreeNode::insert_dfs(root.clone(), i);
        }
        println!("{}", print_tree(root.clone()));
        let root = AVLTreeNode::delete_dfs(root.clone(), 4);
        println!("{}", print_tree(root.clone()));
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    ///
    /// ```text
    ///      3                     3                       3                        2
    ///     / \                   /                       /                        / \
    ///    1   4   delete(4) ->  1    left_rotate(1) ->  2  right_rotate(3)  ->   1   3
    ///     \                     \                     /
    ///      2                     2                   1
    /// ```
    ///
    #[test]
    fn test_delete_left_right() {
        let nums = vec![1, 4, 2];
        let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(3))));
        for i in nums {
            AVLTreeNode::insert_dfs(root.clone(), i);
        }
        println!("{}", print_tree(root.clone()));
        let root = AVLTreeNode::delete_dfs(root.clone(), 4);
        println!("{}", print_tree(root.clone()));
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    /// ```text
    ///        2                     2                         3
    ///       / \                     \                       / \
    ///      1   3   delete(1) ->      3  left_rotate(2) ->  2   4
    ///           \                     \
    ///            4                     4
    /// ```
    #[test]
    fn test_delete_right_right() {
        let nums = vec![1, 3, 4];
        let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(2))));
        for i in nums {
            AVLTreeNode::insert_dfs(root.clone(), i);
        }
        println!("{}", print_tree(root.clone()));
        let root = AVLTreeNode::delete_dfs(root.clone(), 1);
        println!("{}", print_tree(root.clone()));
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }

    /// ```text
    ///        2                     2                         2                            3
    ///       / \                     \                         \                          / \
    ///      1   4   delete(1) ->      4  right_rotate(4) ->     3    left_rotate(2) ->   2   4
    ///         /                     /                           \
    ///        3                     3                             4
    /// ```
    #[test]
    fn test_delete_right_left() {
        let nums = vec![1, 4, 3];
        let root = Some(Rc::new(RefCell::new(AVLTreeNode::new(2))));
        for i in nums {
            AVLTreeNode::insert_dfs(root.clone(), i);
        }
        println!("{}", print_tree(root.clone()));
        let root = AVLTreeNode::delete_dfs(root.clone(), 1);
        println!("{}", print_tree(root.clone()));
        assert!(AVLTreeNode::is_bst(root.clone()));
        assert!(AVLTreeNode::is_balance(root.clone()));
    }
}