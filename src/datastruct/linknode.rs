use std::fmt;

/// 链表节点定义，序列化/反序列化
///
/// 链表节点`ListNode` 有两个字段，其一数据域`val`, 下一个节点`next`。
/// 因链表是递归定义的结构，在编译前需要确定其大小。`next` 声明为 `Option<Box<ListNode>>`。
/// `Box`是智能指针，其包装的数据存储在堆上。
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}


impl ListNode {
    /// `new`是`ListNode`的构造方法，返回`ListNode`实例
    /// #Example
    /// ```
    /// use leetcode_in_rust::datastruct::linknode::ListNode;
    /// let root = ListNode::new(21);
    /// ```
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }


    pub fn create(nums: Vec<i32>) -> Option<Box<Self>> {
        let mut prev = None;
        for &i in nums.iter().rev() {
            let mut node = ListNode::new(i);
            node.next = prev;
            prev = Some(Box::new(node));
        }
        prev
    }

    pub fn creates(nums: Vec<i32>) -> Option<Box<Self>> {
        if nums.len() <= 0 {
            return None;
        }
        // let mut head = ListNode::new(0);
        // let mut node = &mut head;
        //
        // for i in nums {
        //     node.next = Some(Box::new(ListNode::new(i)));
        //     node = node.next.as_mut().unwrap();
        // }
        // head.next

        let mut head = Some(Box::new(ListNode::new(0)));
        let mut node = &mut head;
        for i in nums {
            node.as_mut().unwrap().next = Some(Box::new(ListNode::new(i)));
            node = &mut node.as_mut().unwrap().next;
        }
        head.unwrap().next
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut ans = vec![];
        let mut node = Some(Box::new(self.clone()));
        while let Some(mut cur_node) = node {
            ans.push(cur_node.val);
            node = cur_node.next.take();
        }
        ans
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut node = Some(Box::new(self.clone()));
        while let Some(curnode) = node {
            let n = format!("{}->", curnode.val);
            output.push_str(&n);
            node = curnode.next;
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let nums = vec![1, 2, 3, 4, 5];
        let root = ListNode::create(nums);
        println!("{:?}", root);
    }
}