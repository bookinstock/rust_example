/*

# function

- function

- method
    - self
    - &self
    - &mut self
    - static method

- closure
    - lambda
    - on the fly

        - capture
            - move 
                - T
            - borrow
                - &T, &mutT

        - as input params

            - Fn (&T) ok for &T capture

            - FnMut (&mut T) ok for &T, &mut T capture

            - FnOnce (T)  ok for &T, &mut T , T capture

        - as output params


- higher order functions

    - functional way

- diverging functions

    - !
        - panic!()
        - loop {}
        - exit()
        - continue.


## ps

- ref and to_owned
    - to_owned -> String

- std::iter
- std::iter::Iterator
- std::iter::Iterator::any
- std::iter::Iterator::find
*/

fn main() {
    println!("==============basic============");

    // 和 C/C++ 不一样，Rust 的函数定义位置是没有限制的
    // 我们可以在这里使用函数，后面再定义它
    fizzbuzz_to(100);

    // 一个返回布尔值的函数
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // 边界情况，提前返回
        if rhs == 0 {
            return false;
        }

        // 这是一个表达式，这里可以不用 `return` 关键字
        lhs % rhs == 0
    }

    // 一个 “不” 返回值的函数。实际上会返回一个单元类型 `()`。
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 当函数返回 `()` 时，函数签名可以省略返回类型
    fn fizzbuzz_to(n: u32) {
        for n in 1..n + 1 {
            fizzbuzz(n);
        }
    }

    println!("=============method=============");

    #[derive(Debug)]
    struct  Point {
        x: i32,
        y: i32
    }

    impl Point {
        fn origin() -> Point {
            Point {x:0, y:0}
        }

        fn new(x: i32, y: i32) -> Point {
            Point {x, y}
        }
    }


    let p = Point::origin();

    println!("p= {:?}", p);

    let p = Point::new(1,2);

    println!("p={:?}", p);

    #[derive(Debug)]
    struct Rectangle {
        p1: Point, 
        p2: Point
    }

    impl Rectangle {
        fn area(&self) -> i32 {
            let Point{x: x1, y:y1} = self.p1;
            let Point{x:x2, y: y2} = self.p2;
            ((x1-x2) * (y1-y2)).abs()
        }

        fn perimeter(&self) -> i32 {
            let Point{x: x1, y:y1} = self.p1;
            let Point{x:x2, y: y2} = self.p2;
            2 * ((x1 - x2).abs() + (y1 - y2).abs())
        }


        fn translate(&mut self, x:i32, y:i32){ 
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }

    let mut rect = Rectangle{
        p1: Point::origin(),
        p2: Point::new(1,2),
    };

    println!("rect={:?}", rect);
    println!("rect area={:?}", rect.area());
    println!("rect perimeter={:?}", rect.perimeter());

    rect.translate(5, 5);

    println!("rect={:?}", rect);

    //
    #[derive(Debug)]
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        fn destroy(self) {
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);
        }
    }

    let pair = Pair(Box::new(1), Box::new(2));

    println!("pair={:?}", pair);

    pair.destroy();

    // println!("pair={:?}", pair);

    println!("=============closure=============");

    fn add(n :i32) -> i32 {
        n + 1
    }

    let add_lambda = |n| n + 1; 

    let n = 1;
    println!("add={:?}", add(n));
    println!("add_lambda={:?}", add_lambda(n));

    // // 闭包是匿名的，这里我们将它们绑定到引用。
    // // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    // let closure_annotated = |i: i32| -> i32 { i + 1 };
    // let closure_inferred  = |i     |          i + 1  ;
    
    // // 译注：将闭包绑定到引用的说法可能不准。
    // // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
    // // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
    // // `closure_xxx` 变量解引用。


    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());

    ///

    use std::mem;

    let color = "green";

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 调用闭包，闭包又借用 `color`。
    print();
    print();

    let mut count = 0;

    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    // println!("{:?}", count);
    inc();
    println!("{:?}", count);


    //let reborrow = &mut count;
    // ^ 试一试：将此行注释去掉。
    
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。

    ////

    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
    
    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。

    //////

    fn apply<F>(f: F) where F: FnOnce() {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }

    // use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // println!("{:?}", farewell);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    //
    // fn apply<F>(f: F) where
    //     F: Fn() {
    //     f();
    // }

    // fn main() {
    //     let x = 7;

    //     // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    //     // 将闭包存储到 `print` 中。
    //     let print = || println!("{}", x);

    //     apply(print);
    // }

    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn function() {
        println!("haha");
    }

    let closure = || println!("hehe");

    call_me(function);

    call_me(closure);

    //

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("{:?}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("{:?}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();

    // let x = "ssss".to_owned();

    // let a:i32 = x;

    println!("============iter any============");

    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    println!("any 2 {}", vec1.iter().any(|&x| x == 2));

    println!("any 2 {}", vec1.into_iter().any(|x| x == 2));

    // println!("2 in vec1 {}", );

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。
    println!("2 in array2: {}", array2.iter().any(|&x| x == 2));


    println!("============iter find============");

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec1 的 `iter()` 举出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec2 的 `into_iter()` 举出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。迭代器元素自身
    // 是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));

    println!("=============higher order functions=============");

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let upper= 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared
        }
    }
    println!("imperative style: {}", acc);

    // functional way

    let _sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n*n)
             .take_while(|&n| n<upper)
             .filter(|&n| is_odd(n))
             .fold(0, |sum, i| sum+i);


    println!("range={:?}", (1..4));

    println!("=============diverging functions=============");


    // fn foo() -> ! {
    //     panic!("hello")
    // }

    // foo()

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i%2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));

}
