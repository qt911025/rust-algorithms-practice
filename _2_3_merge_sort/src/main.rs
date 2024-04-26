use algorithms_prelude::CompareSorter;
use merge_sort::MergeSorter;
use std::env;

// 归并排序
// merge sort
// 分而治之

// 总共只会分lgn级，不断除以2直到为1
// 每一级都是一次完整排序，最末一级是两两排序

// 比如升序，就是每次都取两个牌堆顶最小的牌
// 若两个牌堆都是排序过的：
// A牌堆顶若比B牌堆顶小，又因B牌堆顶是B牌堆里最小的牌，所以A牌堆顶比B牌堆所有牌都小
// 又因A牌堆顶是A牌堆最小的牌，所以A牌堆顶是所有牌中最小的牌
// 反之亦然

fn main() {
    let mut int_array: Vec<i32> = env
        ::args()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    MergeSorter(&mut int_array).sort_by(|prev, next| prev < next);

    int_array.iter().for_each(|e| {
        println!("{:?}", e);
    });
}
