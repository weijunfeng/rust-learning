use std::collections::HashMap;

/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

fn main() {
    let nums_one = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum2(nums_one, target);
    println!("result:{result:?}")
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    return vec![];
}

fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut hash_map = HashMap::new();
    for (index, i) in nums.iter().enumerate() {
        let add = target - i;
        if let Some(&other_index) = hash_map.get(&add) {
            return vec![other_index, index];
        }
        hash_map.insert(i, index);
    }
    return vec![];
}
