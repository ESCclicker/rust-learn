

fn main() {
    // let v: Vec<i32> = Vec::new();
    
    // let v1 = vec![1,2,3];
    
    //updating a vector
    let mut v = Vec::new();
    v.push(22);
    v.push(33);
    v.push(2000);
    v.push(2222);
    
    // readding element of vectors
    let third: &i32 = &v[2];
    println!("the third element is {}",third);
    

    let third: Option<&i32> = v.get(2);
    match third { 
        Some(third) => println!("The third element is {third}"),
        None => println!("there is no third element"),
        
    }
    
    // Iterating Over the Values in a Vector
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100,32,45];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }  
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    

    for x in 0..v.len(){
          println!("{}",v[x]);
    }



}