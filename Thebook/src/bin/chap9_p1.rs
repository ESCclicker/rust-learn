struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}





fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someoneusername"),
        email: String::from("someone@yahoo.com"),
        sign_in_count: 1, 
    };
    
    user1.email = String::from("anotheremail@gmail.com");
    println!("username of user1 is {}", user1.username);

    let user2 = build_user(String::from("someoneother@yahoo.com"), String::from("someoneother"));
    println!("username of user2 is {}",user2.username);

    //Creating intances with struct updates syntax
    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@yahoo.com"),
        sign_in_count: user1.sign_in_count,
    };
    
    let _user4 = User {
            email: String::from("otherexample@yahoo.com"),
            ..user2
    };

    // println!("{}",user2.username);  will give Error now 
    // println!("{}",user1.username); 
    // . If we had given _user4 new String values for both email and username, and thus only used the active and sign_in_count values from user1,
    //  then user2 would still be valid after creating _user4. Both active and sign_in_count are types that implement the Copy trait

    
}

fn build_user(email: String, username: String) -> User {
        User {
            active: true, 
            username: username,
            email,
            sign_in_count: 1,
            }
        
}
