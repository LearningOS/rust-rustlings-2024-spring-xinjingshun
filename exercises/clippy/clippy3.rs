// clippy3.rs
// 
// 这里有一些更简单的 Clippy 修正，这样你就可以看到它的实用性。
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM  DONE


use std::mem::swap;
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_vec = vec![1, 2, 3, 4, 5];

    my_vec.clear();
    let my_empty_vec = my_vec;
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
