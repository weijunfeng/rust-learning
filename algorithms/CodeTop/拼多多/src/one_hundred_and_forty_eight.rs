/*
https://leetcode.cn/problems/sort-list/
148. 排序链表
给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

 

示例 1：


输入：head = [4,2,1,3]
输出：[1,2,3,4]
示例 2：


输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]
示例 3：

输入：head = []
输出：[]
 

提示：

链表中节点的数目在范围 [0, 5 * 104] 内
-105 <= Node.val <= 105
 

进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val: val,
        }
    }
    fn append(&mut self, value: i32) {
        let mut current = &mut self.next;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(ListNode::new(value)));
    }
}

struct Solution;

impl Solution {
    // 快慢指针找中点，递归调用排序，合并两个链表
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => head,
            Some(ref node) if node.next.is_none() => head,
            _ => {
                let (left, right) = Self::split_list(head);
                let sorted_left = Self::sort_list(left);
                let sorted_right = Self::sort_list(right);
                Self::merge_lists(sorted_left, sorted_right)
            }
        }
    }

    pub fn merge_lists(left: Option<Box<ListNode>>, right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        let mut l1 = left;
        let mut l2 = right;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                tail.next = l1;
                l1 = tail.next.as_mut().unwrap().next.take();
            } else {
                tail.next = l2;
                l2 = tail.next.as_mut().unwrap().next.take();
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = l1.or(l2);

        dummy.next
    }

    pub fn split_list(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut len = 0;
        let mut ptr = &head;
        while let Some(node) = ptr {
            len += 1;
            ptr = &node.next;
        }
        let mut ptr = &mut head;
        for _ in 0..len / 2 {
            if let Some(ref mut node) = ptr {
                ptr = &mut node.next;
            }
        }
        let next = ptr.take();
        (head, next)
    }
}
// 辅助函数：打印链表
fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = &node.next;
    }
    println!();
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test() {
        let mut head = super::ListNode::new(4);
        head.append(2);
        head.append(1);
        // head.append(3);
        let dd = Some(Box::new(head));
        println!("Original list:");
        super::print_list(&dd);

        let sorted_head = super::Solution::sort_list(dd);

        println!("Sorted list:");
        super::print_list(&sorted_head);
    }
}