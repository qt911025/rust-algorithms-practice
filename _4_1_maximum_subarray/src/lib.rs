// 递归解决，划定一个中点，此时有三种情况：
// 最大子数组在中点左边（递归）
// 在中点右边（递归）
// 跨过中点（终止），中点左边的一半取最大，右边一半取最大，组合起来得最大

use std::i32::MIN;

/**
 * 寻找最大子数组
 */
pub fn find_maximum_subarray(vec: &Vec<i32>, low: usize, high: usize) -> (usize, usize, i32) {
    if vec.len() == 0 {
        (0, 0, 0)
    } else if high - low <= 1 {
        (low, low, vec[low])
    } else {
        // 中点分界左闭右开，参考归并排序的实现
        // 代入的值和返回的值无关，所以这里high和left/right/cross_high不同也没关系
        // 区间是左闭右开，但结果的左右指针是全闭
        let mid = (low + high + 1) >> 1;
        let (left_low, left_high, left_sum) = find_maximum_subarray(vec, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray(vec, mid, high);
        let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(vec, low, mid, high);

        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_high, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_high, cross_sum)
        }
    }
}

fn find_max_crossing_subarray(
    vec: &Vec<i32>,
    low: usize,
    mid: usize,
    high: usize
) -> (usize, usize, i32) {
    let mut left_sum = MIN;
    let mut sum = 0;
    let mut max_left = mid - 1;
    for i in (low..mid).rev() {
        sum += vec[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = MIN;
    sum = 0;
    let mut max_right = mid;
    for j in mid..high {
        sum += vec[j];
        if sum >= right_sum {
            right_sum = sum;
            max_right = j;
        }
    }
    (max_left, max_right, left_sum + right_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        let result = find_maximum_subarray(&vec, 0, vec.len());
        assert_eq!(result, (7, 10, 43)); // 注意下标和书中不同
    }
}
