struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user1 = User { //used mut to allow change of value
        active:true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.username = String::from("usernew");
    println!("User1: {}", user1.username);
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //can do the same thing using struct update syntax
    //let user2 = User{
    //    email: String::from("another@example.com"),
    //    ..user1 //this will take all other values from user1
    //};
    //the struct update moves the dat, just like variables are moved
    //so user1 is no longer valid
    //if we had given user2 new string values for both email and username, and thus only used
    //user1.active and user1.sign_in_count, we could have used user1 after user2 was created
    //both active and sign_in_count are types that implement the Copy trait, so they are copied into user2

}


