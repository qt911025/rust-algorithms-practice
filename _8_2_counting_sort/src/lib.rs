use std::ptr;

pub fn counting_sort<T, F>(
    vec: Vec<T>,
    max_key: usize,
    enumerate: F
) -> Result<Vec<T>, &'static str>
    where F: Fn(&T) -> usize
{
    let mut count = vec![0; max_key];
    for e_ref in &vec {
        let key = enumerate(e_ref);
        if key < max_key {
            count[key] += 1; // 统计同key量
        } else {
            return Err("元素值溢出");
        }
    }

    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    let result_len = vec.len();
    let mut result = Vec::with_capacity(result_len); // 应该定义好容量，而不是new，new出来的vec容量为0
    unsafe {
        result.set_len(result_len);
    }
    for e in vec.into_iter().rev() {
        let key = enumerate(&e);
        unsafe {
            ptr::write(&mut result[count[key] - 1], e); // 注意下标
        }
        count[key] -= 1;
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    const MAX_VALUE: usize = 10;

    use super::*;
    use issort::InsertionSorter;
    use algorithms_prelude::CompareSorter;
    #[test]
    fn it_sort_ascending() {
        let v = vec![2, 4, 1, 7, 9, 9, 5, 5, 2, 4, 2, 3];
        let mut expected = v.clone();
        InsertionSorter(&mut expected).sort_by(|prev, next| prev <= next);
        if let Ok(result) = counting_sort(v, MAX_VALUE, |e| *e) {
            assert_eq!(result, expected);
        } else {
            panic!("测试失败，不应该返回错误");
        }
    }

    #[test]
    fn it_overflow() {
        let v = vec![2, 4, 1, 7, 10];
        let result = counting_sort(v, MAX_VALUE, |e| *e);
        assert_eq!(result, Err("元素值溢出"))
    }

    #[test]
    fn it_struct_sort_ascending() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            id: usize,
            name: &'static str,
        }

        let v = vec![
            Foo {
                id: 9,
                name: "ZS",
            },
            Foo {
                id: 0,
                name: "LS",
            },
            Foo {
                id: 2,
                name: "WW",
            },
            Foo {
                id: 1,
                name: "ZL",
            },
            Foo {
                id: 3,
                name: "SQ",
            }
        ];

        if let Ok(result) = counting_sort(v, MAX_VALUE, |e| e.id) {
            assert_eq!(
                result,
                vec![
                    Foo {
                        id: 0,
                        name: "LS",
                    },
                    Foo {
                        id: 1,
                        name: "ZL",
                    },
                    Foo {
                        id: 2,
                        name: "WW",
                    },
                    Foo {
                        id: 3,
                        name: "SQ",
                    },
                    Foo {
                        id: 9,
                        name: "ZS",
                    }
                ]
            );
        } else {
            panic!("测试失败，不应该返回错误");
        }
    }

    #[test]
    fn it_struct_sort_ascending_box() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            id: usize,
            name: &'static str,
        }

        let v = vec![
            Box::new(Foo {
                id: 9,
                name: "ZS",
            }),
            Box::new(Foo {
                id: 0,
                name: "LS",
            }),
            Box::new(Foo {
                id: 2,
                name: "WW",
            }),
            Box::new(Foo {
                id: 1,
                name: "ZL",
            }),
            Box::new(Foo {
                id: 3,
                name: "SQ",
            })
        ];

        if let Ok(result) = counting_sort(v, MAX_VALUE, |e| e.id) {
            assert_eq!(
                result,
                vec![
                    Box::new(Foo {
                        id: 0,
                        name: "LS",
                    }),
                    Box::new(Foo {
                        id: 1,
                        name: "ZL",
                    }),
                    Box::new(Foo {
                        id: 2,
                        name: "WW",
                    }),
                    Box::new(Foo {
                        id: 3,
                        name: "SQ",
                    }),
                    Box::new(Foo {
                        id: 9,
                        name: "ZS",
                    })
                ]
            );
        } else {
            panic!("测试失败，不应该返回错误");
        }
    }

    #[test]
    fn it_struct_sort_ascending_idempotence() -> Result<(), &'static str> {
        // 幂等性测试
        #[derive(Debug, PartialEq, Clone)]
        struct Foo {
            id: usize,
            name: &'static str,
        }

        let v = vec![
            Foo {
                id: 9,
                name: "ZS",
            },
            Foo {
                id: 0,
                name: "LS",
            },
            Foo {
                id: 2,
                name: "WW",
            },
            Foo {
                id: 1,
                name: "ZL",
            },
            Foo {
                id: 3,
                name: "SQ",
            },
            Foo {
                id: 0,
                name: "LS2",
            }
        ];

        let enumerate = |e: &Foo| e.id;

        let mut expected = v.clone();
        InsertionSorter(&mut expected).sort_by(|prev, next| prev.id <= next.id);

        let result1 = counting_sort(v, MAX_VALUE, enumerate)?;
        assert_eq!(result1, expected);
        let result2 = counting_sort(result1, MAX_VALUE, enumerate)?;
        assert_eq!(result2, expected);
        Ok(())
    }
}
