/*
https://leetcode.cn/problems/reorder-list/description/
143. 重排链表
给定一个单链表 L 的头节点 head ，单链表 L 表示为：

L0 → L1 → … → Ln - 1 → Ln
请将其重新排列后变为：

L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。

 

示例 1：



输入：head = [1,2,3,4]
输出：[1,4,2,3]
示例 2：



输入：head = [1,2,3,4,5]
输出：[1,5,2,4,3]
 

提示：

链表的长度范围为 [1, 5 * 104]
1 <= node.val <= 1000
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
    // 解题思路，快慢指针找链表中点，反转中点后一部分链表，循环拼接前后链表
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut len = 0;
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            len += 1;
            ptr = node.next.as_ref();
        }
        let mut head = head.take();
        let mut ptr = &mut head;
        for _ in 0..len / 2 {
            if let Some(ref mut node) = ptr {
                ptr = &mut node.next;
            }
        }
        let mut mid = ptr.take();
        mid = Self::reverse_list(mid);
        Self::merge_two_lists(head, mid);
    }


    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut current_node) = current {
            current = current_node.next.take();  // 获取下一个节点并将当前节点的 next 置为 None，更新当前节点为下一个节点
            current_node.next = prev;             // 反转链表
            prev = Some(current_node);            // 向后移动
        }
        prev
    }

    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut lhs, mut rhs) = (l1, l2);
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        while let (Some(lnode), Some(rnode)) = (lhs.as_ref(), rhs.as_ref()) {
            if lnode.val <= rnode.val {
                tail.next = lhs;
                tail = tail.next.as_mut().unwrap();
                lhs = tail.next.take();
            } else {
                tail.next = rhs;
                tail = tail.next.as_mut().unwrap();
                rhs = tail.next.take();
            }
        }

        tail.next = if lhs.is_some() { lhs } else { rhs };
        head.next
    }

    fn merge_list(l1: &mut Option<Box<ListNode>>, l2: &mut Option<Box<ListNode>>) {
        let mut p1 = l1.as_mut();
        let mut p2 = l2.take();  // take ownership of the second list

        while let (Some(mut node1), Some(mut node2)) = (p1, p2) {
            p2 = node2.next.take();
            node2.next = node1.next.take();
            node1.next = Some(node2);

            p1 = node1.next.as_mut().and_then(|n| n.next.as_mut());
        }
    }
}