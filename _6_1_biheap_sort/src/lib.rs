use std::ptr;

use algorithms_prelude::CompareSorter;

pub struct BiheapSorter<'a, Seq>(pub &'a mut Seq);

impl<'a, Elem> CompareSorter for BiheapSorter<'a, Vec<Elem>> {
    type Element = Elem;

    fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
        let vec = &mut self.0;

        if vec.len() < 2 {
            return;
        }

        build_max_heap(vec, compare);
        for i in (1..vec.len()).rev() {
            unsafe {
                ptr::swap_nonoverlapping(&mut vec[0], &mut vec[i], 1);
            }
            max_heapify(vec, compare, 0, i);
        }
    }
}

// 构建最大堆，这个只会执行一次
// 构建后，父节点都会大于左右节点，而左右节点之间的大小未定
// i向下取整，因为i不是一个右开区间的右界，而是指向具体下标的“指针”
fn build_max_heap<T>(vec: &mut Vec<T>, compare: fn(prev: &T, next: &T) -> bool) {
    for i in (0..vec.len() >> 1).rev() {
        max_heapify(vec, compare, i, vec.len());
    }
}

fn max_heapify<T>(
    vec: &mut Vec<T>,
    compare: fn(prev: &T, next: &T) -> bool,
    i: usize,
    heap_size: usize
) {
    let l = ((i + 1) << 1) - 1; // 转换成1开头下标，乘以2后再转换成0开头下标
    let r = (i + 1) << 1; // 就在右边
    let mut largest = i;
    if l < heap_size && compare(&vec[largest], &vec[l]) {
        largest = l;
    }
    if r < heap_size && compare(&vec[largest], &vec[r]) {
        largest = r;
    }
    if largest != i {
        unsafe {
            ptr::swap_nonoverlapping(&mut vec[i], &mut vec[largest], 1);
        }
        max_heapify(vec, compare, largest, heap_size);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_sort_ascending() {
        let mut v = vec![22, 43, 145, 1, 9];
        BiheapSorter(&mut v).sort_by(|prev, next| prev < next);
        assert_eq!(v, vec![1, 9, 22, 43, 145]);
    }

    #[test]
    fn it_sort_descending() {
        let mut v = vec![22, 43, 145, 1, 9];
        BiheapSorter(&mut v).sort_by(|prev, next| prev > next);
        assert_eq!(v, vec![145, 43, 22, 9, 1]);
    }

    #[test]
    fn it_struct_sort_ascending() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            id: u32,
            name: &'static str,
        }

        let mut v = vec![
            Foo {
                id: 22,
                name: "ZS",
            },
            Foo {
                id: 43,
                name: "LS",
            },
            Foo {
                id: 145,
                name: "WW",
            },
            Foo {
                id: 1,
                name: "ZL",
            },
            Foo {
                id: 9,
                name: "SQ",
            }
        ];

        BiheapSorter(&mut v).sort_by(|prev, next| prev.id < next.id);
        assert_eq!(
            v,
            vec![
                Foo {
                    id: 1,
                    name: "ZL",
                },
                Foo {
                    id: 9,
                    name: "SQ",
                },
                Foo {
                    id: 22,
                    name: "ZS",
                },
                Foo {
                    id: 43,
                    name: "LS",
                },
                Foo {
                    id: 145,
                    name: "WW",
                }
            ]
        );
    }

    #[test]
    fn it_struct_sort_ascending_box() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            id: u32,
            name: &'static str,
        }

        let mut v = vec![
            Box::new(Foo {
                id: 22,
                name: "ZS",
            }),
            Box::new(Foo {
                id: 43,
                name: "LS",
            }),
            Box::new(Foo {
                id: 145,
                name: "WW",
            }),
            Box::new(Foo {
                id: 1,
                name: "ZL",
            }),
            Box::new(Foo {
                id: 9,
                name: "SQ",
            })
        ];

        BiheapSorter(&mut v).sort_by(|prev, next| prev.id <= next.id);
        assert_eq!(
            v,
            vec![
                Box::new(Foo {
                    id: 1,
                    name: "ZL",
                }),
                Box::new(Foo {
                    id: 9,
                    name: "SQ",
                }),
                Box::new(Foo {
                    id: 22,
                    name: "ZS",
                }),
                Box::new(Foo {
                    id: 43,
                    name: "LS",
                }),
                Box::new(Foo {
                    id: 145,
                    name: "WW",
                })
            ]
        );
    }

    #[test]
    fn it_struct_sort_ascending_idempotence() {
        // 幂等性测试，当比较函数包含等于时，结果应当是幂等的
        #[derive(Debug, PartialEq, Clone)]
        struct Foo {
            id: u32,
            name: &'static str,
        }

        let compare = |prev: &Foo, next: &Foo| prev.id <= next.id;

        let mut v = vec![
            Foo {
                id: 22,
                name: "ZS",
            },
            Foo {
                id: 43,
                name: "LS",
            },
            Foo {
                id: 145,
                name: "WW",
            },
            Foo {
                id: 1,
                name: "ZL",
            },
            Foo {
                id: 9,
                name: "SQ",
            },
            Foo {
                id: 43,
                name: "LS2",
            }
        ];

        BiheapSorter(&mut v).sort_by(compare);
        let sorted_v = v.clone();
        BiheapSorter(&mut v).sort_by(compare);

        assert_eq!(v, sorted_v);
    }
}
