/*
https://leetcode.cn/problems/search-in-rotated-sorted-array/description/
33. 搜索旋转排序数组
整数数组 nums 按升序排列，数组中的值 互不相同 。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。

给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。



示例 1：

输入：nums = [4,5,6,7,0,1,2], target = 0
输出：4
示例 2：

输入：nums = [4,5,6,7,0,1,2], target = 3
输出：-1
示例 3：

输入：nums = [1], target = 0
输出：-1


提示：

1 <= nums.length <= 5000
-104 <= nums[i] <= 104
nums 中的每个值都 独一无二
题目数据保证 nums 在预先未知的某个下标上进行了旋转
-104 <= target <= 104
 */
struct Solution;

impl Solution {
    // 解题思路：
    // 1. 原本数据是有序的，旋转后存在两部分有序
    // 2. 本题如果找到旋转点，即可对两部分使用二分查找，二分查找时间复杂度为 O(log n) 满足题目要求
    // 3. 查找旋转点，可以先把数组取中间点分两部分，肯定存在一部分有序，一部分无序，对有序部分使用二分查找，对无序部分继续使用取中点处理
    // 4. 怎么快速判断两部分的有序性呐，可使用第一个位置值，与中点位置值判断，如果 nums[0]<=nums[mid],则有序，否则该部分无序，另一部分有序
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::get_index(&nums[..], &target);
    }

    pub fn get_index(nums: &[i32], target: &i32) -> i32 {
        if nums.is_empty() || !nums.contains(target) {
            return -1;
        }
        let mid = nums.len() / 2;
        let (ordered_start, ordered_end, disordered_start, disordered_end) = if nums[0] <= nums[mid] {
            (0, mid + 1, mid + 1, nums.len())
        } else {
            (mid, nums.len(), 0, mid)
        };
        let result = nums[ordered_start..ordered_end].binary_search(target);
        if let Ok(index) = result {
            return (index + ordered_start) as i32;
        }
        // 有序部分未找到，继续从无序部分查找
        let result = Solution::get_index(&nums[disordered_start..disordered_end], target);
        if result < 0 {
            result
        } else {
            disordered_start as i32 + result
        }
    }
}