use algorithms_prelude::CompareSorter;
use quick_sort::QuickSorter;
use std::env;

// 快速排序
// 构建一个序列，使得存在某个“主元”，主元左边的比主元小，右边的比主元大
// 最坏时间复杂度为O(n^2)，平均复杂度为O(nlgn)，key越各异，排序越随机，越接近平均情形。
// 反之，越接近已排序，同Key的数据越多，越接近最坏情形。

fn main() {
    let mut int_array: Vec<i32> = env
        ::args()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    QuickSorter(&mut int_array).sort_by(|prev, next| prev < next);

    int_array.iter().for_each(|e| {
        println!("{:?}", e);
    });
}
