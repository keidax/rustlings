// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn main() {
    let x:i32 = 7;
    if x == 10 {
        println!("x is ten!");
    } else {
        let x = 10;
        println!("x is not ten, it's {other_val}!", other_val = x);
    }
}
