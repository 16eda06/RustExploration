#[derive(Debug)]
enum IP {
    V4 (u8, u8, u8, u8),
    V6 (String)
}

pub fn test6_1() {
    let four = IP::V4(127,0,0,1);
    print(&four);
    print(&four);
    let four = IP::V6(String::from("::1"));
    print(&four);
}

fn print(ip_type: &IP) {
    println!("{:?}", ip_type)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }
}

pub fn test6_2() {
    let m = Message::Write(String::from("hello"));
    m.print();
    let m = Message::Quit;
    m.print();
    let m = Message::Move {x:32, y:65};
    m.print();
    let m = Message::ChangeColor(255,255,255);
    m.print();
}