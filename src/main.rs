#![allow(unused_variables)]

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use leetcode_in_rust::datastruct::bin_treenode::{print_tree, TreeNode};


fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }
    if let Some(x) = root {
        let node = x.borrow();
        println!("{}", node.val);
        dfs(&node.left);
        dfs(&node.right);
    }
}

fn main() {
    let nums = vec![
        Some(1),
        Some(2), Some(3),
        Some(4), None, Some(5), Some(6),
        None, Some(7), None, None, Some(8),
    ];
    let root = TreeNode::create(nums); // 1 2 4 7 3 5 6 8
    dfs(&root);
}

