#![allow(unused_variables)]

use leetcode_in_rust::datastruct::treenode::{TreeNode, print_tree};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let nums = vec![Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, Some(7), None, None, Some(8)];
    let root = TreeNode::create(nums.clone());

    println!("{}", print_tree(root.clone()));

    // let mut ans = vec![];
    // let mut node = root.clone();
    // let mut stack = vec![];
    //
    // loop {
    //     while let Some(x) = node{
    //         ans.push(x.borrow().val);
    //         stack.push(x.clone());
    //         node = x.borrow().left.clone();
    //     }
    //     if stack.is_empty() {
    //         break;
    //     }
    //     let x = stack.pop().unwrap();
    //     node = x.borrow().right.clone();
    // }
    // assert_eq!(vec![1, 2, 4, 7, 3, 5, 6, 8], ans);
    // assert_eq!(vec![4, 7, 2, 1, 5, 3, 8, 6], ans);
    // assert_eq!(vec![ 7, 4, 2, 5, 8, 6, 3, 1], ans);
    // assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], ans);

    // let mut ans = vec![];
    // let max_deep = TreeNode::get_height(root.clone());
    //
    // let mut queue = VecDeque::new();
    // queue.push_back(root);
    // let mut deep = 0;
    // while !queue.is_empty() {
    //     let qsize = queue.len();
    //     deep += 1;
    //     for _ in 0..qsize {
    //         match queue.pop_front().flatten() {
    //             Some(x) => {
    //                 ans.push(Some(x.borrow().val));
    //                 queue.push_back(x.borrow().left.clone());
    //                 queue.push_back(x.borrow().right.clone());
    //             }
    //             None  => ans.push(None),
    //         }
    //     }
    // }
    // let size = ans.len();
    // for i in (0..size).rev(){
    //     if ans[i].is_none(){
    //         ans.pop();
    //     } else {
    //         break
    //     }
    // }
    //
    // assert_eq!(nums, ans);
}

