use std::io;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area(),
    );
    let mut height: String = String::new();
    let mut width: String = String::new();

    println!("Enter height.");
    io::stdin()
        .read_line(&mut height)
        .expect("msg");

    println!("Enter width.");
    io::stdin()
        .read_line(&mut width)
        .expect("msg");

    let height: u32 = height.trim().parse().unwrap();
    let width: u32 = width.trim().parse().unwrap();

    let new_rect = Rectangle {
        height,
        width,
    };
    if rect.can_hold(&new_rect) {
        println!("the new rect will fit!");
    } else {
        println!("it wont fit!");
    }
}



fn other_main() {
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("username"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("new_username");
    println!("Username: {}", name);

    let user2 = build_user("email@example.com", "username2");

    let user3 = User {
        email: String::from("blah@foo.com"),
        username: String::from("blah"),
        ..user2
    };


    struct Color(u32, u32, u32);
    struct Point(i32, i32, i32);
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        sign_in_count: 1,
        active: true,
    }
}