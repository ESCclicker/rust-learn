fn main() {
    // creating a new string
    let mut s = String::new();  //empty string s 
    let data = "initital contents";
    let s = data.to_string();
    let s = "contents".to_string(); //also work on a literal directly

    let s = String::from("Initial contents"); //also we can use this to

    // sting are UTF-8 encoded so we can include any properly encoded data in them 
    let hello = String::from("नमस्ते");
    

    // updated a String

    // appending with push_str or push
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); //push_str method takes string slice ( doesn't take ownership)
    println!("we can still use s2 -> {s2}");
    let s2 = 'l';
    s.push(s2); //push method takes a single character as parameter and adds it to the String
    println!("can we still use s2 after it gets entered into push method , lets see -> {s2}");//yes
    

    // Concatenating with `or` `format` 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2 ; // s1 has been moved here ( ownership taken can't used any longer)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3 ; //to complicated to see what's going on 
    //ratther than it we can do -> 
    //firstly not that to that s1 ownership is taken so to use it again we are going to comment out above example or just create new s1
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}"); // also it doesn't take owenership of any

    // indexing into Strings
    
}