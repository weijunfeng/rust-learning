/*
题目描述
"吃货"和"馋嘴"两人到披萨店点了一份铁盘（圆形）披萨，并嘱咐店员将披萨按放射状切成大小相同的偶数个小块。但是粗心的服务员将披萨切成了每块大小都完全不同奇数块，且肉眼能分辨出大小。

由于两人都想吃到最多的披萨，他们商量了一个他们认为公平的分法：从"吃货"开始，轮流取披萨。除了第一块披萨可以任意选取外，其他都必须从缺口开始选。

他俩选披萨的思路不同。"馋嘴"每次都会选最大块的披萨，而且"吃货"知道"馋嘴"的想法。

已知披萨小块的数量以及每块的大小，求"吃货"能分得的最大的披萨大小的总和。

输入描述
第 1 行为一个正整数奇数 N，表示披萨小块数量。

3 ≤ N < 500
接下来的第 2 行到第 N + 1 行（共 N 行），每行为一个正整数，表示第 i 块披萨的大小

1 ≤ i ≤ N
披萨小块从某一块开始，按照一个方向次序顺序编号为 1 ~ N

每块披萨的大小范围为 [1, 2147483647]
输出描述
"吃货"能分得到的最大的披萨大小的总和。

用例
输入	5
8
2
10
5
7
输出	19
说明
此例子中，有 5 块披萨。每块大小依次为 8、2、10、5、7。

按照如下顺序拿披萨，可以使"吃货"拿到最多披萨：

"吃货" 拿大小为 10 的披萨

"馋嘴" 拿大小为 5 的披萨

"吃货" 拿大小为 7 的披萨

"馋嘴" 拿大小为 8 的披萨

"吃货" 拿大小为 2 的披萨

至此，披萨瓜分完毕，"吃货"拿到的披萨总大小为 10 + 7 + 2 = 19

可能存在多种拿法，以上只是其中一种。
 */
use ndarray::Array2;

/// 思路，遍历所有获取方式，取结果的最大值，贪心算法
fn recursive(l: usize, r: usize, pizza: &[i32], cache: &mut Array2<i32>) -> i32 {
    let mut l = l;
    let mut r = r;
    if pizza[l] > pizza[r] {
        l = check(l as i32 - 1, pizza.len());
    } else {
        r = check(r as i32 + 1, pizza.len());
    }
    if cache[[l, r]] > 0 {
        return cache[[l, r]];
    }
    if l == r {
        cache[[l, r]] = pizza[l];
    } else {
        let l_max = recursive(check(l as i32 - 1, pizza.len()), r, pizza, cache) + pizza[l];
        let r_max = recursive(l, check(r as i32 + 1, pizza.len()), pizza, cache) + pizza[r];
        cache[[l, r]] = l_max.max(r_max);
    }
    return cache[[l, r]];
}

fn check(index: i32, total_size: usize) -> usize {
    if index < 0 {
        return total_size - 1;
    }
    if index >= total_size as i32 {
        return 0;
    }
    return index as usize;
}

fn get_max_size(pizza: &[i32]) -> i32 {
    let mut max_size = 0;
    let mut cache = Array2::<i32>::zeros((pizza.len(), pizza.len()));
    for i in 0..pizza.len() {
        let i32 = i32::try_from(i).unwrap();
        let max = recursive(check(i32 - 1, pizza.len()),
                            check(i32 + 1, pizza.len()),
                            pizza, &mut cache) + pizza[i];
        max_size = max_size.max(max);
    }
    max_size
}

#[cfg(test)]
pub mod tests {
    use crate::pizza_fair_division::get_max_size;

    #[test]
    fn test() {
        let dd = [8, 2, 10, 5, 7];
        let result = get_max_size(&dd);
        println!("{result}")
    }
}