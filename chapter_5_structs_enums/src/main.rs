fn main() {
    // STRUCTS
    // how to create a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // create a user1 struct
    let mut user1 = User {
        email: String::from("mrmacro@gmail.com"),
        username: String::from("yodyod"),
        active: true,
        sign_in_count: 1,
    };

    // print the email from the struct user1
    println!("user's email is {}", user1.email);
    // change the username of the struct user1
    user1.username = String::from("John");
    println!("user's name is {}", user1.username);

    // function to create a struct
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(
        String::from("john@email.com"),
        String::from("testing_username"),
    );
    println!(
        "User2 email is: {}, username: {}",
        user2.email, user2.username
    );

    // with tuple
    let rect1 = (30, 50);
    println!(
        "the area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
    // with struct
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect2 is {:#?}", rect2);
    println!(
        "the area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
    // with struct and implementation
    println!(
        "the area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    // new implementation
    println!("can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    // associated function
    let hip_to_be_square = Rectangle::square(42);
    println!("hip_to_be_square {:#?}", hip_to_be_square);

    // ENUMS
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // println!("four is {:#?}", four);
    // println!("six is {:#?}", six);
    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
}

// with tuple
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    // dimensions.0 * dimensions.1
    let (width, height) = dimensions;
    width * height
}

// with struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    // you specify what to do with width and height
    rectangle.width * rectangle.height
}

// ENUMS
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
