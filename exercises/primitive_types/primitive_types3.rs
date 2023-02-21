// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a = ["bottles of beer on the wall"; 100];
    // let a = "asdf2asdf2asdf2asdf2asdfasdf2asdf2asdf2asdf2asdfasdf2asdf2asdf2asdf2asdf222asdf2asdf2asdf2asdf2asdf2";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
