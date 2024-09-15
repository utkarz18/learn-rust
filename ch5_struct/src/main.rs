struct User {
    name: String,
    email: String
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn create_square(side: u32) -> Self {   // associated function without self, usage struct::name_of_function()
        Self {
            width: side,
            height: side
        }
    }
}

fn main() {

    let user1 = User {
        name: String::from("John"),
        email: String::from("John@example.com")
    };

    println!("New user with name {} and email {} created", user1.name, user1.email);

    // user1.name = String::from("Sam"); // reassign if user is mutable

    // let user2 = user1; // assigning user1 to user2 will move all it fields.

    let user2 = User { // this moves all the movable data but keeps the static fields in user1
        name: String::from("Sam"),
        ..user1
    };

    println!("New user with name {} created", user2.name);

    
    let rect1 = Rectangle {
        width: 30,
        height: 20
    };
    println!("Rectangle 1 {:#?}", rect1);
    println!("Area of rectangle 1 : {}", rect1.area());
    dbg!(&rect1);   // another way of printing/debuging a struct type, this macro takes ownership so pass reference
    println!("Creating Square shaped rectangle {:#?}", Rectangle::create_square(10));



    let rect2 = Rectangle {
        width: 15,
        height: 10
    };

    let rect3 = Rectangle {
        width: 35,
        height: 20
    };

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}
