/*
题目描述
现有两组服务器A和B，每组有多个算力不同的CPU，其中 A[i] 是 A 组第 i 个CPU的运算能力，B[i] 是 B组 第 i 个CPU的运算能力。

一组服务器的总算力是各CPU的算力之和。

为了让两组服务器的算力相等，允许从每组各选出一个CPU进行一次交换，

求两组服务器中，用于交换的CPU的算力，并且要求从A组服务器中选出的CPU，算力尽可能小。

输入描述
第一行输入为L1和L2，以空格分隔，L1表示A组服务器中的CPU数量，L2表示B组服务器中的CPU数量。

第二行输入为A组服务器中各个CPU的算力值，以空格分隔。

第三行输入为B组服务器中各个CPU的算力值，以空格分隔。

1 ≤ L1 ≤ 10000
1 ≤ L2 ≤ 10000
1 ≤ A[i] ≤ 100000
1 ≤ B[i] ≤ 100000
输出描述
对于每组测试数据，输出两个整数，以空格分隔，依次表示A组选出的CPU算力，B组选出的CPU算力。

要求从A组选出的CPU的算力尽可能小。

备注
保证两组服务器的初始总算力不同。
答案肯定存在
用例
输入	2 2
1 1
2 2
输出	1 2
说明	从A组中选出算力为1的CPU，与B组中算力为2的进行交换，使两组服务器的算力都等于3。
输入	2 2
1 2
2 3
输出	1 2
说明	无
输入	1 2
2
1 3
输出	2 3
说明	无
输入	3 2
1 2 5
2 4
输出	5 4
说明	无
 */
use std::collections::HashSet;

/// a_sum -a +b = b_sum-b+a ==> b_sum-a_sum = 2*(b-a) ==> b=(b_sum-a_sum)/2+a
fn get_result(a_cpu: &[i32], b_cpu: &[i32]) -> String {
    let a_sum: i32 = a_cpu.iter().sum();
    let mut b_sum = 0;
    let mut b_set = HashSet::new();
    for b in b_cpu {
        b_sum = b_sum + b;
        b_set.insert(b);
    }
    let diff: i32 = (b_sum - a_sum) / 2;
    let mut result = String::new();
    if diff <= 0 {
        return result;
    }
    let mut min_a = i32::MAX;
    for a in a_cpu {
        let b = diff + a;
        if b_set.contains(&b) && min_a >= *a {
            min_a = *a;
            result = format!("{a} {b}");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::get_result(&vec![1, 1], &vec![2, 2]), "1 2");
        assert_eq!(super::get_result(&vec![1, 2], &vec![2, 3]), "1 2");
        assert_eq!(super::get_result(&vec![2], &vec![1, 3]), "2 3");
    }
}