//struct User {
    //active: bool,
    //username: String,
    //email: String,
    //sign_in_count: u64,
//}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//fn main() {
    //let mut user1 = User {
        //active: true,
        //username: String::from("someusername123"),
        //email: String::from("someone@example.com"),
        //sign_in_count: 1,
    //};

    //user1.email  = String::from("anothermail@example.com")
//}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 40
    };

    dbg!(&rect1);
    println!("rect1 is {rect1:#?} and has an area of {}", rect1.area());

    println!("rect2 fits into rect1? {}", rect1.can_hold(&rect2))
}


