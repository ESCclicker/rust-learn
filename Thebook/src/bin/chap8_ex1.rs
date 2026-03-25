//OWNERSHIP AND FUNCTIONS

fn main() {
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s); //s's value move into the function and so is no longer valid here
     // println!("{s}");   -> this will give error as s is no longer valid here
    let x = 5;  //x comes into scope
    makes_copy(x); //because i32 implements the copy trait, x does not move into the function so yeaah it is still valid here and it's okay to use x afterward
    println!("{x}"); //see no error while using it  🦀
} //here, x goes out of scope, then s. however, because s's value was moved,nothing special will happen 




fn  takes_ownership(some_string : String) { // some_string comes into scope 
     println!("{some_string}");

} //here, some_string goes out of scope and 'drop' is called. The backing memory is freed



fn makes_copy(some_integer: i32) {
      println!("{some_integer}");
} //here, some_integer goes out of scope. and nothing else happen 




