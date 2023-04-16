mod ch2;
mod ch3;
mod ch4;
mod ch5;
mod ch6;

use std::io;

fn main() {
    let mut buffer = String::new();
    
    io::stdin()
        .read_line(&mut buffer).expect("Err");
    
    match buffer.trim() {
        "2" => ch2::guessing_game(),
        "3_scope" => ch3::test3_1(),
        "3_tuples" => ch3::test3_2(),
        "3_if" => ch3::test3_3(),
        "3_nested_loop" => ch3::test3_4(),
        "3_for_loop" => ch3::test3_5(),
        "4.1" => ch4::test4_1(),
        "4.2" => ch4::test4_2(),
        "4.3" => ch4::test4_3(),
        "4.3.1" => ch4::test4_3_1(),
        "4.3.2" => ch4::test4_3_2(),
        "4.3.3" => ch4::test4_3_3(),
        "5_area_fn" => ch5::test5_1(),
        "5_area_tup" => ch5::test5_2(),
        "5_area_struct" => ch5::test5_3(),
        "5_area_pd" => ch5::test5_4(),
        "6_enum" => ch6::test6_1(),
        "6_enum2" => ch6::test6_2(),
        &_ => println!("Ignored")
    }
}


