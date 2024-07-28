/*
题目描述
现代计算机系统中通常存在多级的存储设备，针对海量 workload 的优化的一种思路是将热点内存页优先放到快速存储层级，这就需要对内存页进行冷热标记。

一种典型的方案是基于内存页的访问频次进行标记，如果统计窗口内访问次数大于等于设定阈值，则认为是热内存页，否则是冷内存页。

对于统计窗口内跟踪到的访存序列和阈值，现在需要实现基于频次的冷热标记。内存页使用页框号作为标识。

输入描述
第一行输入为 N，表示访存序列的记录条数，0 < N ≤ 10000。

第二行为访存序列，空格分隔的 N 个内存页框号，页面号范围 0 ~ 65535，同一个页框号可能重复出现，出现的次数即为对应框号的频次。

第三行为热内存的频次阈值 T，正整数范围 1 ≤ T ≤ 10000。

输出描述
第一行输出标记为热内存的内存页个数，如果没有被标记的热内存页，则输出 0 。

如果第一行 > 0，则接下来按照访问频次降序输出内存页框号，一行一个，频次一样的页框号，页框号小的排前面。

用例
输入	10
1 2 1 2 1 2 1 2 1 2
5
输出	2
1
2
说明	内存页1和内存页2均被访问了5次，达到了阈值5，因此热内存页有2个。内存页1和内存页2的访问频次相等，页框号小的排前面。
输入	5
1 2 3 4 5
3
输出	0
说明	访存跟踪里面访存频次没有超过3的，因此热内存个数为0
 */
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Frequency {
    count: i32,
    value: i32,
}
impl Frequency {
    fn new(value: i32) -> Self {
        return Frequency {
            count: 1,
            value,
        };
    }
    fn count_increment(&mut self) {
        self.count = self.count + 1
    }
}

fn get_frequency(recode: &str, limit: i32) -> Vec<Frequency> {
    let mut recode_map = HashMap::<i32, Frequency>::new();

    // 处理输入并更新频次
    for num_str in recode.split_whitespace() {
        let recode = num_str.parse::<i32>().unwrap();
        // 找到条目
        recode_map.entry(recode)
            // 修改条目数据
            .and_modify(|freq| freq.count_increment())
            // 条目不存在时插入
            .or_insert_with(|| Frequency::new(recode));
    }

    // 筛选和排序结果
    recode_map.values()
        .filter(|f| f.count >= limit)
        // 把引用转为拥有所有权
        .cloned()
        // 对 iterator排序
        .sorted_by(|f1, f2| f2.count.cmp(&f1.count).then_with(|| f1.value.cmp(&f2.value)))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::memory_page_hotcold_marking::get_frequency;

    #[test]
    fn test() {
        let result = get_frequency("1 2 1 2 1 2 1 2 1 2", 5);
        println!("{result:?}");
        let result = get_frequency("1 2 3 4 5", 3);
        println!("{result:?}");
    }
}