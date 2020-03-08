/*

# conversion

- trait

    - From
        - from

    - Into
        - into

    - TryFrom

        - return result

    - TryInto

        - return result


    - ToString
        - impl fmt::Display
            - println!

    - FromStr
        - parse()


## ps

- std::convert
- std::convert::From
- std::convert::Into
- std::convert::TryFrom
- std::convert::TryInto

- std::string
- std::string::ToString

- std::str
- std::str::FromStr
*/

#[warn(unused_variables)]

fn main() {
    println!("============From and Into===========");

    let my_str = "hello";
    let my_string = String::from(my_str);

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);

    println!("===========TryFrom, TryInto========");

    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("===============ToString, FromStr========");

    use std::string::ToString;

    struct Circle {
        radius: i32,
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {:?}", sum};
}
