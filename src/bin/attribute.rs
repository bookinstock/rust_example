/*

# attribute

- #![crate_attribute]

- #[item_attribute]


#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]

- dead_code

- crate
    - crate_type
    - crate_name

- cfg
    - #[cfg(...)]
    - cfg!(...)
*/

fn main() {
    fn used_function() {}

    // `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
    #[allow(dead_code)]
    fn unused_function() {}

    // #[allow(dead_code)]
    fn noisy_unused_function() {}
    // 改正 ^ 增加一个属性来消除警告

    used_function();
}
