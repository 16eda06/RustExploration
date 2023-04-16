pub fn test5_1() {
    let l1 = 50;
    let w1 = 30;

    println!("The area = {}", area(l1, w1));
}

fn area(l: u32, w: u32) -> u32 {
    l * w
}

struct RectTup (u32, u32, String);

pub fn test5_2() {
    let rect1 = RectTup(50, 30, String::from("My House"));
    println!("The area of {} is = {}", rect1.2, area_tup(&rect1));
}

fn area_tup(dim: &RectTup) -> u32 {
    println!("Calculating the area of {}", dim.2);
    dim.0 * dim.1
}

#[derive(Debug)]
struct Rect {
    l: u32,
    w: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.l * self.w
    }
}

pub fn test5_3() {
    let rect1 = Rect {l:50, w:30};
    println!("rect1 is {:#?}", rect1);
    println!("The area = {}", rect1.area());
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x : f64,
    y : f64
}

impl Point {
    fn distance(&self, other:&Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

pub fn test5_4() {
    let p1 = Point {x:0.0, y:0.0};
    let p2 = Point {x:3.0, y:4.0};
    println!("The distance between \n{:?} \nand \n{:?} \nis = {}", p1, p2 ,p1.distance(&p2));
}