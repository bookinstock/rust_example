/*

# hello

- print
    - println!

- format
    - format!

- display
    - "{}", x

- debug
    - "{:?}", x
    - "{:#?}", x

## ps

- std::fmt
- std::fmt::Binary
- std::fmt::Debug
- std::fmt::Display

*/

fn main() {
    println!("hello");

    println!("=============");

    let x = vec![1, 2, 3, 4, 5];

    for e in &x {
        println!("{:?}", e);
    }

    println!("x={:?}", x);

    // for e in x.iter() {
    //     println!("{:?}", *e);
    // }

    println!("==============");

    try_fmt();

    // try_debug_trait();

    // println!("{:?} months in a year.", 12);

    // println!("{1:?} {0:?} is the {actor:?} name.",
    //          "Slater",
    //          "Christian",
    //          actor="actor's");

    println!("==============");

    try_list();

    // print!("a");

    // println!("a");

    // println!("a={}", 1);

    // eprintln!("a={}", 1);

    // #[derive(Debug)]
    // struct A {
    //     x: isize,
    //     y: usize
    // }

    // let a = A { x: 1, y: 2 };

    // println!("a={:?}", a);

    // println!("a={:#?}", a);

    // let a = format!("a={:#?}", a);

    // println!("{}", a);

    // // trait
    // // fmt::Display -> {}
    // // fmt::Debug -> {:?}

    // println!("{}", format!("Hello"));
    // println!("{}", format!("Hello, {}!", "world"));
    // println!("{}", format!("The number is {}", 1));
    // println!("{}", format!("{:?}", (3, 4)));
    // println!("{}", format!("{value}", value=4));
    // println!("{}", format!("{} {}", 1, 2));
    // println!("{}", format!("{:04}", 42));

    // // comment
    // f();

    // println!("===============");

    // // test()

    // ttt();
}

fn ttt() {
    #[derive(Debug)]
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct Deep(UnPrintable);

    let x = Deep(UnPrintable(18));

    println!("x={:?}", x);
}

/// out
fn f() {
    //! in
}

fn test() {
    use std::fmt;

    #[derive(Debug)]
    struct Vector2D {
        x: isize,
        y: isize,
    }

    impl fmt::Display for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(x->{}, y->{})", self.x, self.y)
        }
    }

    // Different traits allow different forms of output of a type. The meaning
    // of this format is to print the magnitude of a vector.
    impl fmt::Binary for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let magnitude = (self.x * self.x + self.y * self.y) as f64;
            let magnitude = magnitude.sqrt();
            let decimals = f.precision().unwrap_or(3);
            let string = format!("{:.*}", decimals, magnitude);
            f.pad_integral(true, "", &string)
        }
    }

    let myvector = Vector2D { x: 3, y: 4 };
    println!("{}", myvector); // => "(3, 4)"
    println!("{:?}", myvector); // => "Vector2D {x: 3, y:4}"
    println!("{:10.3b}", myvector);

    // println!("x={:?}", x);
}

fn try_debug_trait() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    let origin = Point { x: 0, y: 0 };

    println!("The origin is: {:?}", origin);
}

fn try_list() {
    use std::fmt; // 导入 `fmt` 模块。

    // 定义一个包含单个 `Vec` 的结构体 `List`。
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 使用元组的下标获取值，并创建一个 `vec` 的引用。
            let vec = &self.0;

            write!(f, "[")?;

            // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
            for (count, v) in vec.iter().enumerate() {
                // 对每个元素（第一个元素除外）加上逗号。
                // 使用 `?` 或 `try!` 来返回错误。
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }

            // 加上配对中括号，并返回一个 fmt::Result 值。
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

fn try_fmt() {
    use std::fmt::{self, Display, Formatter};

    struct City {
        name: &'static str,
        // 纬度
        lat: f32,
        // 经度
        lon: f32,
    }

    impl Display for City {
        // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
            // 一个缓冲区（即第一个参数f）中。
            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color)
    }
}
