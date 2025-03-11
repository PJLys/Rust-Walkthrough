#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}



fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", loopback);
    println!("{:#?}", home);

    let some_number = Some(5);
    let some_char = Some('5');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;

    let sum = x + some_number;
}



