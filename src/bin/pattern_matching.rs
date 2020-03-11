

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
}

fn test_match() {
    let a = 1;

    let x = match a {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "???" 
    };

    println!("{}", x);
}

fn test_if_let()  {
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

fn test_literal() {

}

fn test_array() {

}

fn test_enum() {

}

fn test_stuct() {

}

fn test_tuple() {

}

fn test_variable() {

}

fn test_wildcard() {

}

fn test_placeholder() {

}