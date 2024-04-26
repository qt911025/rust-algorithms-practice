struct Bucket<T> {
    head: Node<T>,
}

enum Node<T> {
    Head {
        next: Box<Node<T>>,
    },
    Cons {
        key: f64,
        elem: T,
        next: Box<Node<T>>,
    },
    Nil,
}

impl<T> Bucket<T> {
    fn new() -> Bucket<T> {
        Bucket { head: Node::Head { next: Box::new(Node::Nil) } }
    }

    fn insert(&mut self, elem: T) {}
}

// struct BucketIter<T>(Bucket<T>);
// // 迭代器
// impl<T> IntoIterator for Bucket<T> {
//     type Item = T;
// }

// impl<T> Iterator for BucketIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         mat
//     }
// }
