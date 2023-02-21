// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
    println!("The square of 2 is {}", square(2));
}

fn square(num: i32) -> i32 {
    if num == 2 {
        return num * 2;
    }
    num * num
}
