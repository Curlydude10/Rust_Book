struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("person@place.com"),
        sign_in_count: 1,
    };
    
    let mut user2 = User {
        email: String::from("Different@email.com"),
        ..user1
    };

    println!("User 1 Unsername {}", user2.username);
}
