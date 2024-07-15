use std::fmt::Debug;
use std::ops::Add;

fn main() {

    // Variables and mutability

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");

    const THEE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

    let y = 42;
    let y = y + 1;
    {
        let y = 52;
        println!("the value of y inside is {y}");
    }
    println!("the value of y outside is {y}");

    let spaces = "   ";
    let spaces = spaces.len();


    // Data types

    let guess: u32 = "42aaaaaa".parse().unwrap_or_else(|x| { println!("{x}"); 32 });
    println!("Guess {guess}");
    // let myVal = 42;
    // 42.wrapping_add(30);
    // let c = 42.checked_add(10);
    let i : u32 = 23;
    let f : f32 = 43.2;
    let _d = i as f32 + f;

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("First element is: {}", tup.0);
    println!("Second element is: {}", tup.1);
    println!("Third element is: {}", tup.2);


    // Array Type

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5];

    // Functions
    add(1,2);
    add2(3,4);

}

//Functions
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
fn add2(a: u32, b: u32) -> u32 {
    a + b
}
