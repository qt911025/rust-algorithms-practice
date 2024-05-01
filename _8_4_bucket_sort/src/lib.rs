mod linked_list_bucket;
use linked_list_bucket::*;

use conv::*;

// 桶排序
// 适用于输入元素的值均匀分布在[0,1)，或者能近似线性地建立单射者
// 根据输入的规模n，建立大小为n的桶集
// 输入元素乘以n并向下取整，结果会分布在[0,n)中
// 找到对应的桶，桶是一个链表，元素按线性查找插入排序到桶中
// 将所有桶首尾相接
pub fn bucket_sort<T, F>(arr: Vec<T>, mapper: F) -> Result<Vec<T>, &'static str>
    where F: Fn(&T) -> f64
{
    let arr_length = arr.len();

    // 建桶
    let mut buckets: Vec<Bucket<T>> = Vec::with_capacity(arr_length);
    buckets.resize_with(arr_length, || Bucket::new());

    // 进桶
    for e in arr {
        let key = mapper(&e);
        if key >= 0.0 && key < 1.0 {
            let bucket_id = (key * (arr_length as f64)).approx_as::<usize>().unwrap();
            buckets[bucket_id].insert(key, e);
        } else {
            return Err("元素值溢出");
        }
    }

    // 排序每个桶，并连接
    let result = buckets
        .into_iter()
        .flat_map(|bucket| bucket)
        .collect();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use issort::InsertionSorter;
    use algorithms_prelude::CompareSorter;
    #[test]
    fn it_sort_ascending() {
        let v = vec![0.79, 0.13, 0.16, 0.64, 0.39, 0.2, 0.89, 0.53, 0.71, 0.42];
        let mut expected = v.clone();
        InsertionSorter(&mut expected).sort_by(|prev, next| prev <= next);
        if let Ok(result) = bucket_sort(v, |e| *e) {
            assert_eq!(result, expected);
        } else {
            panic!("测试失败，不应该返回错误");
        }
    }

    #[test]
    fn it_overflow() {
        let v = vec![2, 4, 1, 7, 10];
        let result = bucket_sort(v, |e| *e as f64);
        assert_eq!(result, Err("元素值溢出"))
    }

    #[test]
    fn it_struct_sort_ascending() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            id: f64,
            name: &'static str,
        }

        let v = vec![
            Foo {
                id: 0.9,
                name: "ZS",
            },
            Foo {
                id: 0.0,
                name: "LS",
            },
            Foo {
                id: 0.2,
                name: "WW",
            },
            Foo {
                id: 0.1,
                name: "ZL",
            },
            Foo {
                id: 0.3,
                name: "SQ",
            }
        ];

        if let Ok(result) = bucket_sort(v, |e| e.id) {
            assert_eq!(
                result,
                vec![
                    Foo {
                        id: 0.0,
                        name: "LS",
                    },
                    Foo {
                        id: 0.1,
                        name: "ZL",
                    },
                    Foo {
                        id: 0.2,
                        name: "WW",
                    },
                    Foo {
                        id: 0.3,
                        name: "SQ",
                    },
                    Foo {
                        id: 0.9,
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
            id: f64,
            name: &'static str,
        }

        let v = vec![
            Box::new(Foo {
                id: 0.9,
                name: "ZS",
            }),
            Box::new(Foo {
                id: 0.0,
                name: "LS",
            }),
            Box::new(Foo {
                id: 0.2,
                name: "WW",
            }),
            Box::new(Foo {
                id: 0.1,
                name: "ZL",
            }),
            Box::new(Foo {
                id: 0.3,
                name: "SQ",
            })
        ];

        if let Ok(result) = bucket_sort(v, |e| e.id) {
            assert_eq!(
                result,
                vec![
                    Box::new(Foo {
                        id: 0.0,
                        name: "LS",
                    }),
                    Box::new(Foo {
                        id: 0.1,
                        name: "ZL",
                    }),
                    Box::new(Foo {
                        id: 0.2,
                        name: "WW",
                    }),
                    Box::new(Foo {
                        id: 0.3,
                        name: "SQ",
                    }),
                    Box::new(Foo {
                        id: 0.9,
                        name: "ZS",
                    })
                ]
            );
        } else {
            panic!("测试失败，不应该返回错误");
        }
    }
}
