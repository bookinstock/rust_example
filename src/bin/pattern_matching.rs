/*

# pattern matching

- match

- if-let (else)

- while-let

- literal

- array

- enum

- stuct

- tuple

- variable

- wildcard

- placeholder

- refutable pattern

- irrefutable pattern

*/

fn main() {
    test_match();
    test_if_let();
    test_while_let();
    test_for();

    println!("==========function==========");
    test_function();

    println!("==========tuple==========");

    test_tuple();
}

fn test_match() {
    let a = 1;

    let x = match a {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "???",
    };

    println!("{}", x);
}

fn test_if_let() {
    let a = 1;

    if let 1 = a {
        println!("one!");
    } else {
        println!("???");
    }
}

fn test_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn test_for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn test_function() {
    let x = (1, 2, 3);

    f(&x);

    fn f(&(a, b, c): &(i32, i32, i32)) {
        println!("a={},b={},c={}", a, b, c);
    }
}

fn test_literal() {}

fn test_array() {}

fn test_enum() {}

fn test_stuct() {}

fn test_tuple() {
    let t = (1, 2, 3);

    let (a, b, c) = t;
    println!("a={},b={},c={}", a, b, c);

    let (a, _, c) = t;
    println!("a={},c={}", a, c);

    let (a, ..) = t;
    println!("a={}", a);

    let (.., c) = t;
    println!("c={}", c);
}

fn test_variable() {}

fn test_wildcard() {}

fn test_placeholder() {}
