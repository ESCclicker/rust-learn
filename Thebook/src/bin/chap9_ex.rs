
// program that calculates the area of a rectangle
// fn main() {
//       let width1 = 30;
//       let height1 = 50;

//       println!(
//         "The area of the rectangle is {} sqaure pixels",
//         area(width1,height1)
//       );
// }

// fn area(width:u32, height:u32) -> u32 {
//     width*height
// }


//refactoring with Tuples

// fn main () {
//     let rect1 = (30,50);
//     println!(
//         "The area of the rectangle is {} sqaure pixels",
//         area(rect1)
//     );
// }
// fn area( dimensions: (u32,u32)) -> u32 {
//     dimensions.0*dimensions.1 
    

// }


// Refactoring with Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
       let scale = 2;
       let rect1 = Rectangle {
                 width:dbg!(30*scale),
                 height:50,
       };
       println!(
        "The area of the rectangle is {} sqaure pixels",
        area(&rect1)
       ); 
       println!("rect is {rect1:?}");
       // OR 
       println!("rect is {rect1:#?}");
       // OR we can also use dbg! macro
       dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width*rectangle.height
}

//  The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!