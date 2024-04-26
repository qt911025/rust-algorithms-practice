// use issort::sort;
use algorithms_prelude::CompareSorter;
use issort::InsertionSorter;
use std::env;

// insertion sort
// 插入排序（升序）
// 复杂度n^2（等差数列前n项和）

// ||||||

//  |
// | ||||

// ...

//     |   取出的这个位，根据强归纳法，前面的是已经排序好的，所以只要遇到比自己小的就可以中断比较
// |||| |

//     |
// ||| ||

//     |
// || |||

//   |
// || |||

// ...

// ||||||

fn main() {
    let mut int_array: Vec<i32> = env
        ::args()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    InsertionSorter(&mut int_array).sort_by(|prev, next| prev < next);

    int_array.iter().for_each(|e| {
        println!("{:?}", e);
    });
}
