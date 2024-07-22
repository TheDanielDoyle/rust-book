fn main() {

    // What is ownership

    let _s: &str = "hello";

    let mut _s2: String = String::from("hello");

    _s2.push_str(", world!");

    println!("{_s2}");

    let _s1 = String::from("hello");
    let mut _s2 = _s1.clone();
    _s2.push_str("Dave");

    println!("s1 = {_s1}, s2 = {_s2}");

    let s3: String = gives_ownership();

    println!("{s3}");

    let s4 = takes_and_gives_back(s3);

    println!("{s4}");

    // References and borrowing
    let mut x = String::from("hello ");
    add_something(&mut x);
    println!("{x}");


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
    // println!("{r1}");

    // The Slice Type

    let b: u8 = b' ';

    let s = String::from("hello world");

    let hello = &s[0..5];
    println!("{hello} 123");
    let world = &s[6..11];
    println!("{world} 123");


}

fn add_something(s : &mut String) {
    s.push_str("something")
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}


fn takes_and_gives_back(a_string: String) -> String {
    // a_string
    return a_string;
}
