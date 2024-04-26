use algorithms_prelude::CompareSorter;
use biheap_sort::BiheapSorter;
use std::env;

// 二叉堆排序，大的（排序回调的后者排最高，低的下沉直到叶子）
// 二叉堆的结构和归并排序的调用树一样，都是把高层的节点填满成完全二叉树再继续填叶子，
// 而且规定从左边填起。

fn main() {
    let mut int_array: Vec<i32> = env
        ::args()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    BiheapSorter(&mut int_array).sort_by(|prev, next| prev < next);

    int_array.iter().for_each(|e| {
        println!("{:?}", e);
    });
}
