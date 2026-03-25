fn main() {
    let _reference_to_nothing = dangle();

}

// fn dangle() -> &String {  //dangle return a reference to a string
//          let s = String::from("hello"); //s is a new string

//          &s  // we return a reference to the string ,s


// } // here, s foes out of the scope and is dropped , so its memory goes away
//DANGER

// now to fix it we can do 
// the solution is to return the String directly 

fn dangle() -> String {
         let s = String::from("hello");
         s
}