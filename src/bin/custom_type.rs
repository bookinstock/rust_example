/*

# custom type

- struct
    - c-like struct
    - tuple-like struct
    - unit struct # -> trait
    - deconstruct

- enum
    - use

- constant
    - const
    - static

## ps
- keyword: ref - bind by reference

*/

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段（field）的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    println!("=================struct==============");
    // 使用简单的写法初始化字段，并创建结构体
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", peter);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let new_point = Point { x: 0.1, ..point };

    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    println!("x={}, y={}", point.x, point.y);

    let rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    println!("rectangle={:?}", rectangle);

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("=================enum==============");

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("=================use==============");

    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    println!("=================list==============");

    use List::*;

    enum List {
        // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
        Cons(u32, Box<List>),
        // Nil：末结点，表明链表结束
        Nil,
    }

    // 可以为 enum 定义方法
    impl List {
        // 创建一个空的 List 实例
        fn new() -> List {
            // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
            Nil
        }

        // 处理一个 List，在其头部插入新元素，并返回该 List
        fn prepend(self, elem: u32) -> List {
            // `Cons` 同样为 List 类型
            Cons(elem, Box::new(self))
        }

        // 返回 List 的长度
        fn len(&self) -> u32 {
            // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
            // 取值种类。
            // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
            // 类型要好过匹配引用 `&T`。
            match *self {
                // 不能得到 tail 的所有权，因为 `self` 是借用的；
                // 因此使用一个对 tail 的引用
                Cons(_, ref tail) => 1 + tail.len(),
                // （递归的）基准情形（base case）：一个长度为 0 的空列表
                Nil => 0,
            }
        }

        // 返回列表的字符串表示（该字符串是堆分配的）
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                    // 而不是打印结果到控制台上
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => format!("Nil"),
            }
        }
    }

    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    println!("======================const===================");
    // 全局变量是在在所有其他作用域之外声明的。
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // 在一般函数中访问常量
        n > THRESHOLD
    }

    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}

// 该属性用于隐藏对未使用代码的警告
// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 },
}

// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}
