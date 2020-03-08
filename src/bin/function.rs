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

- higher order functions

- diverging functions

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

    println!("=============higher order functions=============");

    println!("=============diverging functions=============");
}
