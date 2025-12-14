// this removes the warning in the struct definition "fields are never read"
// this warns you that the fields with this warning are never used
#[allow(dead_code)] 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User { active: false, username, email, sign_in_count: 1 }
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = build_user(String::from("user2@example.com"), String::from("username2"));
    user2.username = String::from("user2");

    let _user3 = User {
        username: String::from("user3"),
        email: String::from("user3@example.com"),
        ..user1 // this will use the rest of the values from user1. if only email is specified for
                // example, the email would move from user 1 and invalidate it. In the example here
                // the active and sign_in_count implement the copy trait so user1 is still valid
    };

    let blue = Color(25, 25, 112);
    let _blue_r_value = blue.0;
    let Color(r, g, b) = blue;
    println!("{r}, {g}, {b}");
}
