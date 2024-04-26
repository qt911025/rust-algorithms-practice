use std::env;
use counting_sort::counting_sort;

// 计数排序
// 用于特定情景：
// 排序的key是可数的，对应可数数不会大于某个常数。实际场景中，这个常数要尽量小。
// 比如说这个键总是在[0, 10)区间
fn main() {
    let usize_array: Vec<usize> = env
        ::args()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    match counting_sort(usize_array, 10, |e| *e) {
        Ok(result) => {
            result.iter().for_each(|e| {
                println!("Result is: {:?}", e);
            });
        }
        Err(message) => {
            println!("Error message: {:?}", message);
        }
    }
}
