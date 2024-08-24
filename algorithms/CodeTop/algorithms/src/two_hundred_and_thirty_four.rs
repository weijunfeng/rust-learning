/*
https://leetcode.cn/problems/palindrome-linked-list/description/
234. 回文链表
给你一个单链表的头节点 head ，请你判断该链表是否为
回文链表
。如果是，返回 true ；否则，返回 false 。

 

示例 1：


输入：head = [1,2,2,1]
输出：true
示例 2：


输入：head = [1,2]
输出：false
 

提示：

链表中节点数目在范围[1, 105] 内
0 <= Node.val <= 9
 

进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
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
            val,
        }
    }
}
struct Solution;
impl Solution {
    // 1. 找到中间节点，
    // 2. 从中间节点反转后一半节点， 注意如果是奇数长度链表，反转的一部分不包括中间节点
    // 3. 比较前后两部分的所有节点值是否相同，相同则是回文链表
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut list_len = 0;
        let mut next_node = &head;
        while next_node.is_some() {
            list_len = list_len + 1;
            next_node = &next_node.as_ref().unwrap().next
        }
        let mut middle_node = &head;
        for _i in 0..list_len / 2 {
            middle_node = &middle_node.as_ref().unwrap().next
        }
        // 需要反转的链表，奇数长度，跳过中间节点
        if list_len % 2 != 0 {
            middle_node = &middle_node.as_ref().unwrap().next
        }
        let mut second_node = &Self::reverse(middle_node.clone());
        let mut first_node = &head;
        while let Some(node) = second_node {
            if first_node.as_ref().unwrap().val != node.val {
                return false;
            }
            first_node = &first_node.as_ref().unwrap().next;
            second_node = &second_node.as_ref().unwrap().next;
        }
        true
    }

    pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut current = head;
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        prev
    }
}