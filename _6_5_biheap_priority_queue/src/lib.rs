use std::ptr;

pub struct BiheapPriorityQueue(Vec<i32>);

impl BiheapPriorityQueue {
    fn new() -> BiheapPriorityQueue {
        BiheapPriorityQueue(vec![])
    }

    fn maximum(&self) -> &i32 {
        &self.0[0]
    }

    fn extract_max(&mut self) -> Result<i32, &'static str> {
        let queue = &mut self.0;
        let heap_size = queue.len();
        if heap_size < 1 {
            return Err("heap underflow");
        }
        if heap_size > 1 {
            unsafe {
                ptr::swap(&mut queue[0], &mut queue[heap_size - 1]);
            }
        }
        if let Some(result) = queue.pop() {
            max_heapify(queue, 0);
            Ok(result)
        } else {
            Err("heap underflow")
        }
    }

    fn increase_key(&mut self, mut i: usize, key: i32) -> Result<(), &'static str> {
        // 即使是获得所有权的参数，想要改还得加mut？
        if key < self.0[i] {
            return Err("New key is smaller than current key");
        }
        self.0[i] = key;
        while i > 1 && self.0[i >> 1] < self.0[i] {
            // 上浮
            unsafe {
                ptr::swap(&mut self.0[i], &mut self.0[i >> 1]);
            }
            i = i >> 1;
        }
        Ok(())
    }

    fn insert(&mut self, key: i32) {
        let size = self.0.len();
        self.0.push(key);
        self.increase_key(size - 1, key).unwrap();
    }
}

fn max_heapify(queue: &mut Vec<i32>, i: usize) {
    let l = ((i + 1) << 1) - 1; // 转换成1开头下标，乘以2后再转换成0开头下标
    let r = (i + 1) << 1; // 就在右边
    let mut largest = i;
    if l < queue.len() && queue[l] > queue[largest] {
        largest = l;
    }
    if r < queue.len() && queue[r] > queue[largest] {
        largest = r;
    }
    if largest != i {
        unsafe {
            ptr::swap_nonoverlapping(&mut queue[i], &mut queue[largest], 1);
        }
        max_heapify(queue, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
