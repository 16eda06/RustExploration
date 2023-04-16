fn main() {
    //test1();
    // test2();
    test3();
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn test3() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{:?}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn test2() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn test1() {
    let s = String::from("hello");  // s comes into scope
    let s = takes_ownership(s);     // s's value moves into the function...
                                    // ... and so is no longer valid here
    println!("{}", s);
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{}", x);
}

fn takes_ownership(some_string: String) -> String { // some_string comes into scope
    println!("{}", some_string);
    some_string
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.