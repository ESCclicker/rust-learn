fn main() {
    let s1 = String::from("hello"); //s1 entered the scope

    let (s2, len) = calculate_length(s1); //s1 is now moved to calculate_length and returns value 

    println!("The length of '{s2}' is {len}."); 
}

fn calculate_length(s: String) -> (String, usize) {  //takes ownership of s1 and return value are String and integer(usize)
    let length = s.len(); // len() returns the length of a String

    (s, length)  //returns the ownership of string along with it length
}

//But this is too much ceremony and a lot of work for a concept that should be common.
// Luckily for us, Rust has a feature for using a value without transferring ownership: references.