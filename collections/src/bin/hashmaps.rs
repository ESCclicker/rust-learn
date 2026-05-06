use std::collections::HashMap;
fn main() {
    // Creating a new HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"), 50);
    // Accessing Values in a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");
    for (key,value) in &scores {
        println!("{key}:{value}");
    }

    // Managing Ownership in HashMaps
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    //field_name and field_value are invalid after this point (ownership taken)
    // below both will error if tried to use them 
    // println!("{field_name}");
    // println!("{field_value}");
    //if we will insert reference values (&fieldname) ownership will not be taken

    // Updating a HashMap 
      //overwriting a Value
    //currently team Blue score is 10
    scores.insert(String::from("Blue"),25);
    //now it is 25 
    
    //adding a key and value only if key isn't present 
    scores.entry(String::from("Red")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("{scores:?}"); //output will be {"Blue": 25, "Red": 40, "Yellow": 50}
    // as Red was already not a Key it got value added
    //Blue was already a key , it got checked and no value added
    
    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut note = HashMap::new();
    for word in text.split_whitespace() {
        let count = note.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{note:?}");
    
    
}



