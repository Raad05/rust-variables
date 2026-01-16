fn main() {
    // immutable variable
    let x = 10;
    println!("Value of x: {}", x);

    // mutable variable
    let mut y: i32 = 10;
    println!("Before change: {}", y);

    y = 5;
    println!("After change: {}", y);

    // constant variable
    const HOUR_IN_SECONDS: u32 = 1 * 60 * 60;
    println!("Number of seconds in an hour: {}", HOUR_IN_SECONDS);
}
