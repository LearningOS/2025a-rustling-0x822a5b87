// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:ident) => {
        println!("I'm an identifier: value = {}", $val);
    };
    ($val:expr) => {
        println!("Look at this other macro: value = {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!("Generics");
    let x = 10;
    my_macro!(x);
}
