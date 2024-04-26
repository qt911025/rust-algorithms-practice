use std::{ mem::{ forget, ManuallyDrop }, ptr };
use algorithms_prelude::CompareSorter;

pub struct MergeSorter<'a, Seq>(pub &'a mut Seq);

impl<'a, Elem> CompareSorter for MergeSorter<'a, Vec<Elem>> {
    type Element = Elem;

    fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
        let vec = &mut self.0;

        if vec.len() < 2 {
            return;
        }

        unsafe {
            let p = vec.as_mut_ptr().cast::<ManuallyDrop<Elem>>();
            let len = vec.len();
            let cap = vec.capacity();
            let mut temp = Vec::from_raw_parts(p, len, cap);
            merge_sort(&mut temp, vec, compare, 0, vec.len());
            forget(temp);
        }
    }
}

fn merge_sort<T>(
    temp: &mut Vec<ManuallyDrop<T>>,
    vec: &Vec<T>,
    compare: fn(prev: &T, next: &T) -> bool,
    p: usize,
    r: usize
) {
    if r - p > 1 {
        let q = (p + 1 + r) >> 1; // 为了让左子树先大，整个域右移一格（或者说是结果向上取整）
        unsafe {
            let mut left = Vec::<ManuallyDrop<T>>::with_capacity(q - p);
            left.set_len(q - p);
            merge_sort(&mut left, vec, compare, p, q);

            let mut right = Vec::<ManuallyDrop<T>>::with_capacity(r - q);
            right.set_len(r - q);
            merge_sort(&mut right, vec, compare, q, r);
            merge(temp, compare, left, right);
        }
    } else {
        // 叶子，终止
        unsafe {
            ptr::copy((&vec[p] as *const T).cast::<ManuallyDrop<T>>(), &mut temp[0], 1);
        }
    }
}

fn merge<T>(
    temp: &mut Vec<ManuallyDrop<T>>,
    compare: fn(prev: &T, next: &T) -> bool,
    left: Vec<ManuallyDrop<T>>,
    right: Vec<ManuallyDrop<T>>
) {
    let left_length = left.len();
    let right_length = right.len();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    unsafe {
        while i < left_length && j < right_length {
            if compare(&left[i], &right[j]) {
                ptr::copy(&left[i], &mut temp[k], 1);
                i += 1;
            } else {
                ptr::copy(&right[j], &mut temp[k], 1);
                j += 1;
            }
            k += 1;
        }

        if i < left_length {
            ptr::copy(&left[i], &mut temp[k], left_length - i);
        } else if j < right_length {
            ptr::copy(&right[j], &mut temp[k], right_length - j);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_sort_ascending() {
        let mut v = vec![22, 43, 145, 1, 9];
        MergeSorter(&mut v).sort_by(|prev, next| prev < next);
        assert_eq!(v, vec![1, 9, 22, 43, 145]);
    }

    #[test]
    fn it_sort_descending() {
        let mut v = vec![22, 43, 145, 1, 9];
        MergeSorter(&mut v).sort_by(|prev, next| prev > next);
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

        MergeSorter(&mut v).sort_by(|prev, next| prev.id < next.id);
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
    fn it_struct_sort_ascending_equal() {
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
            },
            Foo {
                id: 43,
                name: "LS2",
            }
        ];

        MergeSorter(&mut v).sort_by(|prev, next| prev.id <= next.id);
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
                    id: 43,
                    name: "LS2",
                },
                Foo {
                    id: 145,
                    name: "WW",
                }
            ]
        );
    }

    #[test]
    fn it_struct_sort_ascending_equal_box() {
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
            }),
            Box::new(Foo {
                id: 43,
                name: "LS2",
            })
        ];

        MergeSorter(&mut v).sort_by(|prev, next| prev.id <= next.id);
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
                    id: 43,
                    name: "LS2",
                }),
                Box::new(Foo {
                    id: 145,
                    name: "WW",
                })
            ]
        );
    }
}
