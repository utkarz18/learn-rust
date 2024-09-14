fn main() {
    struct User {
        name: String,
        email: String,
        is_active: bool,
        sign_in_count: u64,
    }

    let mut user1 = User {
        name: String::from("John"),
        email: String::from("John@example.com"),
        is_active: true,
        sign_in_count: 1
    };

    println!("New user with name {} created", user1.name);

    // user1.name = String::from("Sam"); // reassign if user is mutable

    // let user2 = user1; // assigning user1 to user2 will move all it fields.

    let user2 = User { // this moves all the movable data but keeps the static fields in user1
        name: String::from("Sam"),
        ..user1
    };

    println!("New user with name {} created", user2.name);
    println!("User 1 name {}", user1.sign_in_count);
}
