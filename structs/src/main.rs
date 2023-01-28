#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user1);

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    println!("{:?}", user2);

    let user3 = build_user(String::from("email@example.com"), String::from("username"));

    println!("{:?}", user3);

    let user4 = User {
        email: String::from("someotheremail@example.com"),
        ..user1
    };

    println!("{:?}", user4);

    let black = Color(0, 0, 0);
    println!("{:?}", black);

    let origin = Point(0, 0, 0);
    println!("{:?}", origin);

    let subject = AlwaysEqual;
    println!("{:?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
