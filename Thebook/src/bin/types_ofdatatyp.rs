
/*
datatypes in rust 
scaler :-> 
rust have four primary scaler types 
Intergers
Floating Point numbers
Booleans
Characters

Compound Types 
Tuples
Arrays

*/

fn main() {
    let _num1  = 22 ; //default to i32 
    let _num2: u32 = 22 ; // unsigned u32
    

    //floating point types
    let _x = 2.0; //f64 by default
    let _x2 :f32 = 3.0 ; //f32
    

    // *******SOME NUMERIC OPERATIONS*********
    //addition
    let _sum = 5+10 ; //15
    //subtraction
    let _difference = 95.5 - 5.5 ; //90
    //multiplication
    let _product = 4*3; //12
    // division 
    let _quotient = 56.7/33.2 ; // 1.7078313253012047 
    let _truncated = -5/3 ; // results in -1 
    let _seeanswer = -5./3. ; //-1.6666666666666667
    // remainder 
    let _remainder = 43 % 5 ; // 3
    
    boolean();
    Character();
}

//let create a new funtion for rest of code ( lmao i learned this from AI hehe)

fn boolean() {
    println!("hehe");
    let _t = true; //rust infers the type 
    let _f:bool = false; // with explicit type annotation 
}
fn Character(){

}
