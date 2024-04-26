// 定义一个Sorter Trait
// 建议实现Sorter Trait的是一个Wrapper
// sort_by一个断言函数，定义的是前一个与后一个元素满足断言函数的关系。
// | 比如传入一个大于关系的函数gt()，排序后，前一个应比后一个大
// | 那么gt的定义为
// | fn gt(prev:i32, next:i32) {
// |     prev > next
// | }
// 使得这个排序的结果为降序
// Sorter获取原序列的可变引用，以sort_by改变原序列
pub trait CompareSorter {
    type Element;
    fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct InsertionSorter<'a, Seq>(&'a mut Seq);

    impl<'a, Elem: Copy> CompareSorter for InsertionSorter<'a, Vec<Elem>> {
        type Element = Elem;

        fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
            let vec = &mut self.0;
            let len = vec.len();

            if len < 2 {
                return;
            }

            for i in 1..vec.len() {
                let e = vec[i];
                let mut j = i;
                while j > 0 && !compare(&vec[j - 1], &e) {
                    vec[j] = vec[j - 1];
                    j = j - 1;
                }
                vec[j] = e;
            }
        }
    }

    #[test]
    fn it_sort_ascending() {
        let mut v = vec![22, 43, 145, 1, 9];
        InsertionSorter(&mut v).sort_by(|prev, next| prev < next);
        assert_eq!(v, vec![1, 9, 22, 43, 145]);
    }
}
