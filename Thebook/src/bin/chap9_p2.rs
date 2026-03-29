// Tuple structs
struct Color(i32,i32, i32);
struct Point(i32, i32, i32);


struct AlwaysEqual;


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); 
    let Point(x, y, z) = origin;  //to destructure the values in the origin point into variables named x, y, and z
    let Color(a, b, c) =  black; 
    println!("{} {} {} {} {} {} ", x,y,z,a,b,c);
    //or access like this
    println!("{}", black.2); 
    
    // defining unit-like structs
    let _subject = AlwaysEqual;
    

}


