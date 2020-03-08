fn main() {
    println!("=============if else===============");
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // 这个表达式返回一个 `i32` 类型。
        10 * n
    } else {
        println!(", and is a big number, half the number");

        // 这个表达式也必须返回一个 `i32` 类型。
        n / 2
        // 试一试 ^ 试着加上一个分号来结束这条表达式。
    };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n);

    println!("=============for===============");

    let mut count = 0u32;

    println!("lets count util infinity");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{:?}", count);

        if count == 5 {
            println!("ok, that;s enough");

            break;
        }
    }

    println!("================nested===========");

    'outer: loop {
        println!("enter outer loop");

        'inner: loop {
            println!("enter inner loop");

            break 'outer;
        }

        println!("never reach");
    }

    println!("exit outer loop");

    println!("===============break==============");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result={:?}", result);

    println!("===============while==============");

    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else {
            println!("{:?}", n);
        }

        n += 1;
    }

    println!("=================for ===============");

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else {
            println!("{:?}", n);
        }
    }

    for n in 1..=101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else {
            println!("{:?}", n);
        }
    }

    let names = vec!["wende", "zhaobo", "junda"];

    for name in names.iter() {
        match name {
            &"wende" => println!("me!"),
            name => println!("my friend {}!", name),
        }
    }

    let names = vec!["wende", "zhaobo", "junda"];

    for name in names.into_iter() {
        match name {
            "wende" => println!("me!"),
            name => println!("my friend {}!", name),
        }
    }

    //error
    // println!("names={:?}", names);

    let mut names = vec!["wende", "zhaobo", "junda"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "wende" => "lu",
            &mut "zhaobo" => "laobo",
            _ => "kinda",
        }
    }

    println!("names={:?}", names);

    println!("=============match============");

    let number = 3;
    println!("tell me about {}", number);

    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime!"),
        13..=19 => println!("a teen"),
        _ => println!("ain't special"),
    };

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("binary={:?}", binary);

    match (0, 1, 2) {
        (a, b, c) => println!("a+b+c={:?}", a + b + c),
    }

    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 18, 32);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }

    let reference = &1;

    match reference {
        &val => println!("got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref is_a_reference = 3;

    println!("val ={:?}", *is_a_reference);

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("got a reference to a value {:?}", r),
    }

    println!("value={:?}", value);

    match mut_value {
        ref mut m => {
            *m += 10;
        }
    }
    println!("mut_value={:?}", mut_value);

    match mut_value {
        mut _m => {
            _m += 10;
        }
    }
    println!("mut_value={:?}", mut_value);

    struct Foo {
        x: (i32, i32),
        y: i32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y: c } = foo;
    println!("a ={},b={},c={}", a, b, c);

    let Foo { y: a, x: b } = foo;

    println!("x={:?},y={:?}", a, b);

    let Foo { x: a, .. } = foo;

    println!("x={:?}", a);

    let Foo { x, y } = foo;

    println!("x={:?},y={:?}", x, y);

    let pair = (2, 2);

    let result = match pair {
        (x, y) if x < y => "x less than y",
        (x, y) if x > y => "x greater than y",
        (x, y) if x == y => "x eq to y",
        _ => "wtf",
    };

    println!("result={}", result);

    fn age() -> u32 {
        15
    }

    let result = match age() {
        0 => "I'm not born yet I guess".to_string(),
        n @ 1..=12 => format!("I'm a child of age {:?}", n),
        n @ 13..=19 => format!("I'm a teen of age {:?}", n),
        n => format!("I'm an old person of age {:?}", n),
    };

    println!("result= {:?}", result);

    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        Some(n @ 42) => println!("the answer is {}", n),
        Some(n) => println!("not interest {:?}", n),
        None => println!("None"),
    }

    println!("================if let==============");

    let x = Some(1);
    match x {
        Some(n) => println!("n={:?}", n),
        None => println!("None"),
    }

    if let Some(n) = x {
        println!("n= {:?}", n);
    } else {
        println!("xxxx");
    }

    println!("====================while let============");

    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("greater than 9");
                    optional = None;
                } else {
                    println!("i={:?}, try again", i);
                    optional = Some(i + 1);
                }
            }
            None => break,
        }
    }
    /////

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9");
            optional = None;
        } else {
            println!("i={:?}, try again", i);
            optional = Some(i + 1);
        }
    }
}

/*

# control flow

- if else, match

    - match

        - a

        - a | b | c

        - a..=b


        - destructure
            - tuple

            - enum

            - struct

            - pointer

        - guard
            - if

        - bind
            - @

- loop, while, for

    - break, continue

        - break return val

    - for e in x
        - Iterator
            - into_iter (implicity)
            - iter, iter_mut (explicity)
                - iter (ref - borrow)
                - iter_mut (mut ref - borrow)

- if let

- while let

## ps
- pointer
    - destructure => &, ref, ref mut
    - dereference => *
*/
