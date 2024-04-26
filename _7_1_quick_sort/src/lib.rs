use std::ptr;

use algorithms_prelude::CompareSorter;

pub struct QuickSorter<'a, Seq>(pub &'a mut Seq);

impl<'a, Elem> CompareSorter for QuickSorter<'a, Vec<Elem>> {
    type Element = Elem;

    fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
        let vec = &mut self.0;

        if vec.len() < 2 {
            return;
        }

        quick_sort(vec, compare, 0, vec.len());
    }
}

fn quick_sort<T>(
    vec: &mut Vec<T>,
    compare: fn(prev: &T, next: &T) -> bool,
    first: usize,
    end: usize
) {
    if end - first > 1 {
        // 用相同的末尾开区间原则，避免usize在0的情况下-1（即使是safe代码，这还是会panic）
        let divider = partrition(vec, compare, first, end);
        quick_sort(vec, compare, first, divider);
        quick_sort(vec, compare, divider + 1, end);
    }
}

fn partrition<T>(
    vec: &mut Vec<T>,
    compare: fn(prev: &T, next: &T) -> bool,
    first: usize,
    end: usize
) -> usize {
    // 随机选一个主元，让划分更平均，但这样强行换位置，就做不到幂等了
    // 而且最后一个元素的大小本来就是随机的，所以再随机并没有意义
    // let random_pivot = rand::thread_rng().gen_range(first..end);
    let last = end - 1;
    // unsafe {
    //     ptr::swap_nonoverlapping(&mut vec[random_pivot], &mut vec[last], 1);
    // }
    let mut i = first;
    for j in first..last {
        // 最后一个是待换的
        if compare(&vec[j], &vec[last]) {
            unsafe {
                ptr::swap_nonoverlapping(&mut vec[i], &mut vec[j], 1);
            }
            i += 1;
        }
    }
    unsafe {
        ptr::swap_nonoverlapping(&mut vec[i], &mut vec[last], 1);
    }
    i
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_sort_ascending() {
        let mut v = vec![22, 43, 145, 1, 9];
        QuickSorter(&mut v).sort_by(|prev, next| prev < next);
        assert_eq!(v, vec![1, 9, 22, 43, 145]);
    }

    #[test]
    fn it_sort_descending() {
        let mut v = vec![22, 43, 145, 1, 9];
        QuickSorter(&mut v).sort_by(|prev, next| prev > next);
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

        QuickSorter(&mut v).sort_by(|prev, next| prev.id < next.id);
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

        QuickSorter(&mut v).sort_by(|prev, next| prev.id <= next.id);
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

        QuickSorter(&mut v).sort_by(compare);
        let sorted_v = v.clone();
        QuickSorter(&mut v).sort_by(compare);

        assert_eq!(v, sorted_v);
    }
}
