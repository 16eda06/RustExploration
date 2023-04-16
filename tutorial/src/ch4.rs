pub fn test4_1() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{:?}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn test4_2() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn test4_3() {
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

pub fn test4_3_1() {
    let mut buffer = String::from("Hello World");
    let word = first_word(&buffer);
    println!("{}", word);    
    buffer.clear();
}

pub fn test4_3_2() {
    let my_string = String::from("hello world");

    let _word = first_word(&my_string[..6]);
    println!("{}",_word);
    let _word = first_word(&my_string[..]);
    println!("{}",_word);
    let _word = first_word(&my_string);
    println!("{}",_word);
    
    let my_string_literal = "hello world";

    let _word = first_word(&my_string_literal[..6]);
    println!("{}",_word);
    let _word = first_word(&my_string_literal[..]);
    println!("{}",_word);
    
    let _word = first_word(my_string_literal);
    println!("{}",_word);
}

pub fn test4_3_3() {
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

