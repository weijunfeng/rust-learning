/*
快速排序
基本思想：通过一趟排序将待排记录分隔成独立的两部分，其中一部分记录的关键字均比另一部分的关键字小，则可分别对这两部分记录继续进行排序，以达到整个序列有序

使用分治法来把一个串（list）分为两个子串（sub-lists）。具体算法描述如下：
- 从数列中挑出一个元素，称为 “基准”（pivot）；
- 重新排序数列，所有元素比基准值小的摆放在基准前面，所有元素比基准值大的摆在基准的后面（相同的数可以到任一边）。
在这个分区退出之后，该基准就处于数列的中间位置。这个称为分区（partition）操作；
- 递归地（recursive）把小于基准值元素的子数列和大于基准值元素的子数列排序。
 */
use std::fmt::Debug;

fn quick_sort<T: PartialOrd + Debug>(arr: &mut [T], start: usize, end: usize) {
    if arr.len() <= 1 {
        return;
    }
    let end = end.min(arr.len() - 1);
    if start >= end {
        return;
    }
    let partition = partition(arr, start, end);
    quick_sort(arr, partition + 1, end);
    if partition > 0 {
        quick_sort(arr, start, partition - 1);
    }
}

fn partition<T: PartialOrd + Debug>(arr: &mut [T], start: usize, end: usize) -> usize {
    // arr[end]作为基准点数据，遍历过程中不能变，只有在最后变更
    // let mut left = start;
    // let mut right = end - 1;
    // while left <= right {
    //     if arr[left] <= arr[end] {
    //         left += 1;
    //         continue;
    //     }
    //     if arr[right] > arr[end] {
    //         if right == 0 {
    //             break;
    //         }
    //         right -= 1;
    //         continue;
    //     }
    //     arr.swap(left, right);
    //     left += 1;
    //     if right == 0 {
    //         break;
    //     }
    //     right -= 1;
    // }
    // arr.swap(left, end);
    // println!("index:{left},start:{start},edn:{end}， {arr:?}");
    // left
    // 预期基准点位置
    let mut pivot_index = start;
    for i in start..end {
        if arr[i] < arr[end] {
            // 数据比基准点小，交换数据到预期基准点位置，此时预期基准点位置被占用，预期基准点位置+1；
            arr.swap(i, pivot_index);
            pivot_index += 1;
        }
    }
    arr.swap(end, pivot_index);
    println!("index:{pivot_index},start:{start},edn:{end}， {arr:?}");
    pivot_index
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        let end = empty_vec.len();
        super::quick_sort(&mut empty_vec, 0, end);
        assert_eq!(empty_vec, Vec::<String>::new());
    }
    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        let end = vec.len();
        super::quick_sort(&mut vec, 0, end);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        let end = vec.len();
        super::quick_sort(&mut vec, 0, end);
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