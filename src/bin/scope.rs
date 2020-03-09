/*

# scope

- RAII

- ownership
    
- move

- borrow

    - immut

    - mut

    - freeze

    - alias

    - ref pattern

- lifetime

    - explicit annotation

    - function

    - method

    - struct

    - trait

    - bound

    - coercion

    - static

    - elision

## ps:

- valgrind : check memory leak [http://valgrind.org/info/]

*/

fn main() {
    println!("=======drop======");

    test_drop();

    println!("========move=======");

    test_move();

    test_box_mut();


    println!("========borrow========");

    test_borrow();

    test_borrow_mut();

    test_borrow_freeze();

    test_borrow_alias();

    test_borrow_ref();

    println!("========lifetime=========");

    test_lifetime();

    test_lifetime_annotation();
}

fn test_drop() {
    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let x = ToDrop;
    println!("Made a ToDrop!");
}

fn test_move() {
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // `c` 被销毁且内存得到释放
    }

    // 栈分配的整型
    let x = 5u32;

    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *移动* `a` 到 `b`
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。
    
    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    //println!("a contains: {}", a);
    // 试一试 ^ 去掉此行注释

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
    // 报错！和前面出错的原因一样。
    //println!("b contains: {}", b);
    // 试一试 ^ 去掉此行注释


}

fn test_box_mut() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

fn test_borrow() {
    // 此函数取得一个 box 的所有权并销毁它
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // 此函数借用了一个 i32 类型
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
    // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
    // 在函数中临时创建的引用也就不复存在了。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错！
        // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
        // eat_box_i32(boxed_i32.clone);
        // 改正 ^ 注释掉此行

        // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
        //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` 离开作用域且不再被借用。
    }

    // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
    //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
    eat_box_i32(boxed_i32);
}

fn test_borrow_mut() {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        // `&'static str` 是一个对分配在只读内存区的字符串的引用
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    // 此函数接受一个对 Book 类型的引用
    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edition", book.title, book.year);
    }

    // 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }   

    // 创建一个名为 `immutabook` 的不可变的 Book 实例
    let immutabook = Book {
        // 字符串字面量拥有 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
    let mut mutabook = immutabook;
    
    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);
    
    // 可变地借用一个可变对象
    new_edition(&mut mutabook);
    
    // 报错！不能可变地借用一个不可变对象
    // new_edition(&mut immutabook);
    // 改正 ^ 注释掉此行
}

fn test_borrow_freeze() {
    let mut _mutable_integer = 7i32;

    {
        // 借用 `_mutable_integer`
        let large_integer = &_mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        
        // 改正 ^ 注释掉此行

        println!("Immutably borrowed {}", large_integer);

        _mutable_integer = 50;

        // `large_integer` 离开作用域
    }

    println!("x={:?}", _mutable_integer);

    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    _mutable_integer = 3;

    println!("x={:?}", _mutable_integer);
}

fn test_borrow_alias() {
    struct Point { x: i32, y: i32, z: i32 }

    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // 通过引用和原始所有者来访问数据
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // 报错！不能可变地借用 `point` ，因为现在它有不可变的借用。
        //let mutable_borrow = &mut point;
        // 试一试 ^ 取消此行注释。

        // 此处再次使用被借用的值
        println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

        // 不可变引用离开作用域
    }

    {
        let mutable_borrow = &mut point;

        // 通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错！不能不可变地借用 `point`，因为现在它有可变的借用。
        // let y = &point.y;
        // 试一试 ^ 取消此行注释。

        // 报错！不能打印，因为 `println!` 会创建一个不可变引用。
        //println!("Point Z coordinate is {}", point.z);
        // 试一试 ^ 取消此行注释。

        // 可以工作！可变引用可以作为不可变的传给 `println!`。
        println!("Point has coordinates: ({}, {}, {})",
                 mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // 可变引用离开作用域
    }

    // 现在又可以不可变地借用 `point` 了。
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             borrowed_point.x, borrowed_point.y, borrowed_point.z);


}

fn test_borrow_ref() {
    #[derive(Clone, Copy)]
    struct Point { x: i32, y: i32 }

    let c = 'Q';

    // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
    let ref ref_c1 = c;

    // assert_eq!(ref_c1, &c);


    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    println!("ref_c1 equals ref_c2: {}", ref_c1 == ref_c2);

    let point = Point { x: 0, y: 0 };

    // 在解构一个结构体时 `ref` 同样有效。
    let (_copy_of_x, _copy_of_x2) = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Point { x: ref ref_to_x, y: _ } = point;

        // 返回一个 `point` 的 `x` 字段的拷贝。
        
        (*ref_to_x, *&point.x)
    };
    println!("XXXXXXXXXX={}-{}", _copy_of_x, _copy_of_x2);

    // `point` 的可变拷贝
    let mut mutable_point = point;

    {
        // `ref` 可以与 `mut` 结合以创建可变引用。
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // 通过可变引用来改变 `mutable_point` 的字段 `y`。
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);


    let mut mutable_tuple = (Box::new(5u32), 3u32);
    
    {
        let (ref mut first, ref mut last) = mutable_tuple;
        **first = 10u32;
        *last = 2u32;
    }
    
    println!("tuple is {:?}", mutable_tuple);


}

fn test_lifetime() {
    let mut x = 1;

    {
        let ref_x = &x;
    }

    {
        let mut ref_x = &mut x;

        *ref_x += 1;
    }

    println!("x={}", x);
}

fn test_lifetime_annotation() {
    // `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`。
    // 这两个生命周期都必须至少要和 `print_refs` 函数一样长。
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    // 不带参数的函数，不过有一个生命周期参数 `'a`。
    fn failed_borrow<'a>() {
        let _x = 12;

        // 报错：`_x` 的生命周期不够长
        //let y: &'a i32 = &_x;
        // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
        // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
    }

    // 创建变量，稍后用于借用。
    let (four, nine) = (4, 9);
    
    // 两个变量的借用（`&`）都传进函数。
    print_refs(&four, &nine);
    // 任何被借用的输入量都必须比借用者生存得更长。
    // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长。
    
    failed_borrow();
    // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
    // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。

}


//
fn ps_ref(){
    let c = 'Q';

    let ref ref_c1 = c;
    // let ref mut ref_c1 = c;

    assert_eq!(ref_c1, &c);
    assert_eq!(*ref_c1, *&c);
}