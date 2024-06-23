// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

// I AM DONE

#[allow(unused_variables, unused_assignments)]
#[allow(clippy::panicking_unwrap)]
#[allow(clippy::almost_swapped)]
#[allow(clippy::unnecessary_literal_unwrap)]
#[allow(clippy::let_unit_value)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap();
    }

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    value_a = value_b;
    value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
