// Tuple structs
struct Color(i32,i32, i32);
struct Point(i32, i32, i32);


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); 
    let Point(x, y, z) = origin;
    let Color(a, b, c) =  black; 
    println!("{} {} {} {} {} {} ", x,y,z,a,b,c);
    //or access like this
    println!("{}", black.2); 

    


}


