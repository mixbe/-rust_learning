/***
冒泡排序：
 */
fn bubble_sort_v1<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    for _ in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            println!("{:?}", arr);
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/*
优化后的冒泡排序
 */
fn bubble_sort_v2<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            println!("{:?}", arr);
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[test]
pub fn test_bubble_sort_v1() {
    // 5 * 4 * 3 * 2 * 1 = 120
    let mut vec1 = vec![2, 7, 9, 1, 4, 6];
    bubble_sort_v1(&mut vec1);
    println!("{:?}", vec1);
}

#[test]
pub fn test_bubble_sort_v2() {
    // 5 * 4 * 3 * 2 * 1 = 120
    let mut vec1 = vec![2, 7, 9, 1, 4, 6];
    bubble_sort_v2(&mut vec1);
    println!("{:?}", vec1);
}


/*
选择排序（大大减少了交换次数）
先从未排序列表选择一个最小值，放在已排序最后一个；
 */
fn select_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();
    for left_index in 0..len {
        println!("{:?}", arr);
        let mut smallest_index = left_index;
        for right_index in (left_index + 1)..len {
            if arr[right_index] < arr[smallest_index] {
                smallest_index = right_index;
            }
        }
        arr.swap(left_index, smallest_index);
    }
}

#[test]
pub fn test_select_sort() {
    // 5 * 4 * 3 * 2 * 1 = 120
    let mut vec1 = vec![2, 7, 9, 1, 4, 6];
    select_sort(&mut vec1);
    println!("{:?}", vec1);
}