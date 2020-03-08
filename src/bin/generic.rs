/*

# generic

- function

- struct

- impl

- trait

- trait bound

    - empty bound

    - multi bound

- where

- newtype

- associated item

- q1 存在问题

- q2 关联类型


## ps

- use std::marker::PhantomData; ???

*/

use std::fmt::Debug;

fn f<T: Debug>(a: T) {
    println!("hello, {:?}", a);
}

#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
}

fn main() {
    f(1);
    f(A { x: 1, y: 2 });

    //

    #[derive(Debug)]
    struct SingleGen<T>(T);

    let a = SingleGen(1);
    let b = SingleGen('a');
    let c = SingleGen(A { x: 1, y: 2 });
    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);

    println!("============fucntion===========");

    function();

    println!("============fucntion===========");

    implementaion();

    println!("============trait============");

    test_trait();

    println!("=============trait bound=======");

    test_bound();

    println!("=============where=============");

    test_where();

    println!("=============newtype===========");

    test_newtype();

    println!("==============q1=============");

    q1();

    println!("==============q2=============");

    q2();
}

fn function() {
    struct A; // 具体类型 `A`。
    struct S(A); // 具体类型 `S`。
    struct SGen<T>(T); // 泛型类型 `SGen`。

    // 下面全部函数都得到了变量的所有权，并立即使之离开作用域，将变量释放。

    // 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
    // 因为没有 `<T>` 这样的泛型类型参数，所以这不是泛型函数。
    fn reg_fn(_s: S) {}

    // 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
    // `SGen<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
    // 泛型类型参数，所以函数不是泛型的。
    fn gen_spec_t(_s: SGen<A>) {}

    // 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
    // `SGen<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
    // 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
    fn gen_spec_i32(_s: SGen<i32>) {}

    // 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
    // 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
    fn generic<T>(_s: SGen<T>) {}

    // 使用非泛型函数
    reg_fn(S(A)); // 具体类型。
    gen_spec_t(SGen(A)); // 隐式地指定类型参数 `A`。
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数 `i32`。

    // 为 `generic()` 显式地指定类型参数 `char`。
    generic::<char>(SGen('a'));

    // 为 `generic()` 隐式地指定类型参数 `char`。
    generic(SGen('c'));
}

fn implementaion() {
    struct S; // 具体类型 `S`
    struct GenericVal<T>(T); // 泛型类型 `GenericVal`

    // GenericVal 的 `impl`，此处我们显式地指定了类型参数：
    impl GenericVal<f32> {} // 指定 `f32` 类型
    impl GenericVal<S> {} // 指定为上面定义的 `S`

    // `<T>` 必须在类型之前写出来，以使类型 `T` 代表泛型。
    impl<T> GenericVal<T> {}

    ////

    #[derive(Debug)]
    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    // Val 的 `impl`
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    // let val = Val {val: 12.0};
    // println!("val={:?}", val);

    // GenVal 的 `impl`，指定 `T` 是泛型类型
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}

fn test_trait() {
    // 不可复制的类型。
    struct Empty;
    struct Null;

    // `T` 的泛型 trait。
    trait DoubleDrop<T> {
        // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
        fn double_drop(self, _: T);
    }

    // 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
    impl<T, U> DoubleDrop<T> for U {
        // 此方法获得两个传入参数的所有权，并释放它们。
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    // 释放 `empty` 和 `null`。
    empty.double_drop(null);

    //empty;
    //null;
    // ^ 试一试：去掉这两行的注释。
}

use std::fmt::Display;

fn test_bound() {
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    struct S<T: Display>(T);

    // 报错！`Vec<T>` 未实现 `Display`。此次泛型具体化失败。
    let s = S(1);

    ////

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }
    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    // 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
    // 都可以让下面函数正常工作。
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    // `T` 必须实现 `HasArea`。任意符合该约束的泛型的实例
    // 都可访问 `HasArea` 的 `area` 函数
    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ 试一试：取消上述语句的注释。
    // | 报错：未实现 `Debug` 或 `HasArea`。

    ////

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}", t);
        println!("u: `{:?}", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // 试一试 ^ 将此行注释去掉。

    compare_types(&array, &vec);
}

fn test_where() {
    //     impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

    //     // 使用 `where` 从句来表达约束
    //     impl <A, D> MyTrait<A, D> for YourType where
    //         A: TraitB + TraitC,
    //         D: TraitE + TraitF {}

    trait PrintInOption {
        fn print_in_option(self);
    }

    // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
    // 或着改用另一种间接的方法。
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
        // 否则我们会给出错误的约束。
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];

    vec.print_in_option();
}

fn test_newtype() {
    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        /// 舍去不满一年的部分
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}

// A, B 的类型 由 C 决定
fn q1() {
    struct Container(i32, i32);

    // 这个 trait 检查给定的 2 个项是否储存于容器中
    // 并且能够获得容器的第一个或最后一个值。
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // 显式地要求 `A` 和 `B`
        fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
        fn last(&self) -> i32; // 未显式地要求 `A` 或 `B`
    }

    impl Contains<i32, i32> for Container {
        // 如果存储的数字和给定的相等则为真。
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // 得到第一个数字。
        fn first(&self) -> i32 {
            self.0
        }

        // 得到最后一个数字。
        fn last(&self) -> i32 {
            self.1
        }
    }

    // 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦。
    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn q2() {
    // trait Contains {
    //     type A;
    //     type B;

    //     // 这种语法能够泛型地表示这些新类型。
    //     fn contains(&self, &Self::A, &Self::B) -> bool;
    // }

    // 不使用关联类型
    // fn difference<A, B, C>(container: &C) -> i32 where
    //     C: Contains<A, B> { ... }

    // // 使用关联类型
    // fn difference<C: Contains>(container: &C) -> i32 { ... }

    ///

    struct Container(i32, i32);

    // 这个 trait 检查给定的 2 个项是否储存于容器中
    // 并且能够获得容器的第一个或最后一个值。
    trait Contains {
        // 在这里定义可以被方法使用的泛型类型。
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
        // 为 `Container(i32, i32)`，那么 `output`（输出）类型
        // 会被确定为 `i32` 和 `i32`。
        type A = i32;
        type B = i32;

        // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // 得到第一个数字。
        fn first(&self) -> i32 {
            self.0
        }

        // 得到最后一个数字。
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
