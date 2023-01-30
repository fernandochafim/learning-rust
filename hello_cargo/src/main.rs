use rand::random;

fn main() {
    let mut y = 10;
    let x:u8 = 8; //Variables are immutable.
    const AGE:i32 = 25;
    println!("X value is: {}", x);
    println!("Initially Y value is: {}", y);
    println!("Constant AGE is: {}", AGE);
    y = 5;
    let x:u8 = x + x; //Change a immutable variable is called shadowing.
    println!("In the end, Y value is: {}", y);
    println!("In the end, X value is: {}", x);
    let number:u8 = random();
    println!("Hello, world! Your number is: {}", number);
}
