struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){

    let user1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user = {}", user1.email);
}