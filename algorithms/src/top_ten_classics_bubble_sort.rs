/// 冒泡排序，从第一个数开始往下冒泡
///它重复地走访过要排序的数列，一次比较两个元素，如果它们的顺序错误就把它们交换过来。走访数列的工作是重复地进行直到没有再需要交换，也就是说该数列已经排序完成。
/// 这个算法的名字由来是因为越小的元素会经由交换慢慢“浮”到数列的顶端。
/// 
/// 使用两个循环，第一个循环保证遍历所有位置，第二个循环处理每个数据都参与到数据比较

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) -> usize {
    let mut cycles = 0;
    let size = arr.len();
    for i in 0..size {
        // 用于标记本次循环是否发生数据交换，如果没有则表明已全部有序，不用再继续循环
        let mut swapped = false;
        // 最后的 i 个数据已经是有序的，不用参与第二次循环的数据比较
        for j in 1..size - i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        cycles = i;
        if !swapped {
            break;
        }
    }
    return cycles;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        let cycles = super::bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
        assert_eq!(0, cycles);
    }
    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        let cycles = super::bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
        assert_eq!(8, cycles);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        super::bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}
