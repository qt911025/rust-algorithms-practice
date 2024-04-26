use std::ptr;
use algorithms_prelude::CompareSorter;

// 1.直接实现：需要Copy Trait，如果只是做基础数据类型不用考虑太多

// 2. std::mem的几个函数是用来绕过麻烦的所有权转让规则的
// 哪怕是移动，将数组的元素调个位置都非常麻烦
// 这么做的好处是，不需要实现Copy了（Copy实际上要求实现深拷贝），但需要实现Default，或者要求用Option包裹（因为没有null）。

// 要清楚哪些环节是“没有银弹”的，设计了一个比较泛用的接口，但是实现不泛用，要一个个实现。
// 在这里，序列如何排序是要各自实现的，而元素类型是交给钩子函数实现的，所以元素类型泛用，序列类型不泛用
pub struct InsertionSorter<'a, Seq>(pub &'a mut Seq);

// impl<'a, Elem: Default> Sorter for InsertionSorter<'a, Vec<Elem>> {
//     // type Sequence = Vec<Elem>;
//     type Element = Elem;

//     fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
//         let vec = &mut self.0; // 可变借用了self.0，只改变vec里的元素，而vec本身没有被赋予其他值，不需要声明为可变变量
//         let len = vec.len();

//         if len < 2 {
//             return;
//         }

//         // struct Foo(i32);
//         // let arr = [Foo(1), Foo(2), Foo(3)];
//         // let elm = arr[1]; // 这不能移动出来

//         for i in 1..vec.len() {
//             let e = vec[i];
//             // 错误：cannot move out of index of Vec<Elem>
//             // 这涉及到数组的本质，数组（元组、结构体）是一个指针，对下标（结构体则是成员）的访问本质上是在该地址基础上偏移
//             // 偏移的量不用操心，编译器会根据指针的数据类型和下标决定好
//             // 那么在Rust里，数组内容、成员，依然是一种基于引用的寻址访问，它可以读，可以写，但不能移动，除非实现了Copy，也就是只能复制

//             // 归根结底，整体的一部分不能被移动出来，只有引用才行
//             let mut j = i;
//             while j > 0 && !compare(&vec[j - 1], &e) {
//                 vec[j] = vec[j - 1];
//                 j = j - 1;
//             }
//             vec[j] = e;
//         }
//     }
// }

// impl<'a, Elem: Default> Sorter for InsertionSorter<'a, Vec<Elem>> {
//     // type Sequence = Vec<Elem>;
//     type Element = Elem;

//     fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
//         let vec = &mut self.0; // 可变借用了self.0，只改变vec里的元素，而vec本身没有被赋予其他值，不需要声明为可变变量
//         let len = vec.len();

//         if len < 2 {
//             return;
//         }

//         for i in 1..vec.len() {
//             let e = mem::take(&mut vec[i]);
//             let mut j = i;
//             while j > 0 && !compare(&vec[j - 1], &e) {
//                 let t = mem::take(&mut vec[j - 1]);
//                 vec[j] = t;
//                 j = j - 1;
//             }
//             vec[j] = e;
//         }
//     }
// }

// 3. mem是安全封装，安全的代码不能有null所以要么有default要么有option
// 但是现在要做的就只是调换个位置
// 现在，改成unsafe实现去掉Default限制
// impl<'a, Elem> Sorter for InsertionSorter<'a, Vec<Elem>> {
//     // type Sequence = Vec<Elem>;
//     type Element = Elem;

//     fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
//         let vec = &mut self.0;
//         // 可变借用了self.0，只改变vec里的元素，而vec本身没有被赋予其他值，不需要声明为可变变量
//         // 因为没有内部可变模式，层层改变必须每一层都可变借用
//         let len = vec.len();

//         if len < 2 {
//             return;
//         }

//         // println!("Size of element is: {:?}", mem::size_of::<Self::Element>());
//         // println!("Size of i32 is: {:?}", mem::size_of::<i32>());

//         for i in 1..vec.len() {
//             unsafe {
//                 // 不安全的read & write直接读写
//                 let e = ptr::read(&vec[i]);
//                 let mut j = i;
//                 while j > 0 && !compare(&vec[j - 1], &e) {
//                     // read write还要多写一道，不如直接copy
//                     // let t = ptr::read(&vec[j - 1]);
//                     // ptr::write(&mut vec[j], t);

//                     // ptr::copy对标c的memmove，ptr::copy_nonoverlapping对标c的memcpy
//                     // count是自动识别拷贝所指对象的类型大小，设置拷贝几格
//                     ptr::copy(&vec[j - 1], &mut vec[j], 1);
//                     j = j - 1;
//                 }
//                 ptr::copy(&e, &mut vec[j], 1); // 前一个是copy后一个自然也是copy，不安全的不要和安全的混用
//                 mem::forget(e); // 标记e不进行drop
//             }
//         }
//     }
// }

// 那么，不用ptr包的函数，直接赋值可以吗？
// 并不能，哪怕是unsafe也不允许
// unsafe只是允许使用raw pointer，raw pointer的值本身可以为null，可以无视多可变引用限制。除此之外没别的了
// raw pointer依然不能违反借用规则，使用raw pointer指向的目标依然不能将未实现Copy的目标值复制出来
// 而ptr::copy的实现已经是调用了rust内核实现了（rust-intrinsic）

// 4. 子循环寻找与目标比较的过程是一个顺序查找的过程，但整个插入排序是一个递归过程，
//    对于每一个vec[i]，vec[0]..=vec[i-1]是一个已排序的序列，所以可以用二分查找法以O(lgn)的速度查找。
// 最终这个插入排序的复杂度是O(nlgn)
impl<'a, Elem> CompareSorter for InsertionSorter<'a, Vec<Elem>> {
    type Element = Elem;

    fn sort_by(&mut self, compare: fn(prev: &Self::Element, next: &Self::Element) -> bool) {
        let vec = &mut self.0;
        let len = vec.len();

        if len < 2 {
            return;
        }

        // println!("Size of element is: {:?}", mem::size_of::<Self::Element>());
        // println!("Size of i32 is: {:?}", mem::size_of::<i32>());

        for i in 1..vec.len() {
            unsafe {
                let e = ptr::read(&vec[i]);
                let mut left = 0;
                let mut right = i;
                while left < right {
                    let j = (left + right) >> 1; // floor((left + right) / 2)
                    // 两个自然数的和除以2的结果只能是自然数或自然数 + 0.5，无符号整型去掉小数部分就是向下取整
                    // （注意有符号整型，正值去掉小数部分是向下取整，负值则是向上取整）
                    // 除以2等价于右移一位，右移计算速度比除以更快（不过即使写/2编译器也会优化的）
                    // 以上计算的结果是，j∈[left, right)，只有在left == right时j == right

                    // 同之前的实现一样，e永远是在与自己左边的元素比较，
                    // 所以j的定位都假设自己在e的左方。
                    // right的初始值为i而且j在循环体内取不到right也符合这个假设
                    if compare(&vec[j], &e) {
                        // 所以当符合比较时，vec[j]以及j之前的元素都在e的左方
                        // 而右方尚未确定，所以论域的左界移到j + 1，避免vec[j]再次被取到（j是可能等于left的）
                        left = j + 1;
                    } else {
                        // 同理，不符合比较条件时，vec[j]以及j之后的元素都在e的右方
                        // j的论域右界移到j，因为论域是右开区间，vec[j]以及j之后的不再会被取到
                        right = j;
                    }
                }
                if i - left > 0 {
                    // 当i - left为0时，大多数情况是不会执行的，但是left+1可能会等于vec.len()导致这行崩溃
                    ptr::copy(&vec[left], &mut vec[left + 1], i - left); // 这时候left == right了
                }
                ptr::write(&mut vec[left], e); //write对应read，不需要forget e因为write会把e吃掉
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_sort_ascending() {
        let mut v = vec![22, 43, 145, 1, 9];
        InsertionSorter(&mut v).sort_by(|prev, next| prev < next);
        // sort(&mut v);
        assert_eq!(v, vec![1, 9, 22, 43, 145]);
    }

    #[test]
    fn it_sort_descending() {
        let mut v = vec![22, 43, 145, 1, 9];
        InsertionSorter(&mut v).sort_by(|prev, next| prev > next);
        // sort(&mut v);
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

        InsertionSorter(&mut v).sort_by(|prev, next| prev.id < next.id);
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

        InsertionSorter(&mut v).sort_by(|prev, next| prev.id <= next.id);
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

        // 为什么 <= 能保证原始顺序不变，因为只有不符合比较条件的才会被移动（才算逆序对），等于应当是符合比较条件的。
        // 要让等于的对子不会被移动，就要将其视为符合比较条件
        InsertionSorter(&mut v).sort_by(|prev, next| prev.id <= next.id);
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
