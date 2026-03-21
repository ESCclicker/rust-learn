
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
use std::io;

fn main() {
    //_tuple();
    //_array();
    invalid_array_ele_access();
      
}

fn _integer_float(){
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
    
} 
    
    

//let create a new funtion for rest of code ( lmao i learned this from AI hehe)


fn _boolean() {
    println!("hehe");
    let _t = true; //rust infers the type 
    let _f:bool = false; // with explicit type annotation 
}
fn _character(){
        let _c = 'x';
        let  _z: char = 'Z' ;
        let _peace_bird = "🕊️";
        
}

fn _tuple(){
    //in tuple elements can have different types
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    //or 
    let _tup = (500,6.4,0); //hey see by using 'let' we changed the value of _tup ( it is still immutable )
                            //it is like assigning a new variable with same name (still doubts ?discussed earlier ,it's just u are dumb 🦀 )
    //let (x,y, z) = tup ; 
    //println!("the value of x,y,z is : {x},{y},{z}"); // the value of x,y,z is : 500,6.4,1
    let five_hundred = tup.0; //accessing the value using tuple_name.index
    let _six_point_four = tup.1 ;
    let _one = tup.2 ;
    println!("{five_hundred}");
}

fn _array(){
    //every element in array must have the same type and it have fixed length 
    let _a = [1,2,3,4,5,7];
    let _a : [i32;4] = [2,3,4,5]; // [i32(type the array will contain); 4(no. of elements in arrat )]
    // also to initilize an array which contain same value for each element we can do 
    let _a = [3;5]; // a = [3,3,3,3,3]
    
    //accessing the element in array
    let _a = [1,2,3,4,5];
    let first = _a[0];
    let _second = _a[1]; // name_of_array[index]
    print!("{first}");
}

fn invalid_array_ele_access(){
            let a = [1,2,3,4,5];
            println!("enter an array index");
            let mut index = String::new() ;
            io::stdin()
                .read_line(&mut index)
                .expect("failed to read line");
            let index: usize = index 
                 .trim()
                 .parse()
                 .expect("index enetered was not a number");
            let element = a[index];
            println!("the value of element at index {index} is :{element}");

            /* if we tried to enter a index out of bound in above code
            rust will give a error like index out of bound
            note----> it shows that rust will check that index we have entered , is it less
            than the length of array 
            if it greater than or equal to array length then rust will panic
            it is something like this ------> index_given < len(array)
            if yes then we can access memory if not then no memery access
            unlike some other low level languages 
            */
    

}