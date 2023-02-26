struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email:String, username:String) -> User{
    User{
        active: true,
        username, //don't need to write username:username, as both key value names are same
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User { //used mut to allow change of value
        active:true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.username = String::from("usernew");
    let mut user2 = build_user(String::from("hello@example.com"), String::from("hello"));
    println!("User1: {}", user1.username);
    println!("User2: {}", user2.username);
}

