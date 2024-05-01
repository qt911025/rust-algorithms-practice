pub struct Bucket<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    key: f64,
    elem: T,
    next: Link<T>,
}

impl<T> Bucket<T> {
    pub fn new() -> Bucket<T> {
        Bucket { head: None }
    }

    // key升序插入，顺序查询
    pub fn insert(&mut self, key: f64, elem: T) {
        let mut new_node = Box::new(Node { key, elem, next: None });
        match &mut self.head {
            None => {
                // 空桶
                self.head = Some(new_node);
            }
            Some(head_node) => {
                if head_node.key > new_node.key {
                    //插入开头
                    new_node.next = self.head.take();
                    self.head = Some(new_node);
                } else {
                    // 顺序查询
                    unsafe {
                        let mut cur_node = head_node as *mut Box<Node<T>>;
                        while let Some(next_node) = &mut (*cur_node).next {
                            if next_node.key > new_node.key {
                                break;
                            } else {
                                cur_node = next_node;
                            }
                        }
                        new_node.next = (*cur_node).next.take();
                        (*cur_node).next = Some(new_node);
                    }
                }
            }
        }
    }
}

// 递归释放变循环释放
impl<T> Drop for Bucket<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut box_node) = cur_link {
            cur_link = box_node.next.take();
        }
    }
}

pub struct BucketIter<T>(Bucket<T>);

impl<T> IntoIterator for Bucket<T> {
    type Item = T;
    type IntoIter = BucketIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BucketIter(self)
    }
}

// 迭代器 只实现夺取所有权的
impl<T> Iterator for BucketIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // 弹出
        self.0.head.take().map(|node| {
            self.0.head = node.next;
            node.elem
        })
    }
}
