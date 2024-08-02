/*
https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
19. 删除链表的倒数第 N 个结点
给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

 

示例 1：


输入：head = [1,2,3,4,5], n = 2
输出：[1,2,3,5]
示例 2：

输入：head = [1], n = 1
输出：[]
示例 3：

输入：head = [1,2], n = 1
输出：[1]
 

提示：

链表中结点的数目为 sz
1 <= sz <= 30
0 <= Node.val <= 100
1 <= n <= sz
 

进阶：你能尝试使用一趟扫描实现吗？
 */
// use std::sync::Mutex;
// 
// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
// 
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val,
//         }
//     }
// }
// static cur: Mutex<i32> = Mutex::new(0);
// struct Solution;
// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         // Self::remove_nth_from_end(head, n);
//         // Solution::cur = Solution::cur+1;
//         // if Solution::cur == n{
//         //     return head.next;
//         // }
//         return head;
//     }
//     pub fn remove_nth_from_end2<'a>(head: &'a mut Option<Box<ListNode>>, n: i32) -> &'a Option<Box<ListNode>> {
//         head.as_mut().unwrap().next = Self::remove_nth_from_end2(head, n);
//         let mut dd = cur.lock().unwrap();
//         *dd = *dd + 1;
//         if *dd == n {
//             return &head.as_ref().unwrap().next;
//         }
//         return head;
//     }
// }