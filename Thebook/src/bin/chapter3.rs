fn main() {
    let mut x = 5;
    x = x - 1;
    
    let x = x + 1;

    {
        let mut x = x * 2;

        println!("the value of x in the inner scope is : {x}");
        x = 4;
        println!("{x}");
    }

    println!("the value of x is : {x}");

}

fn _shadowing_use() {
       
    let spaces = "    ";
    let spaces = spaces.len();
    
    println!("spaces to give are {spaces}")
    //here we changed the type of spaces with the help of shadowing ( by again assigning the spaces variable 
    // we overshadowed the older spaces variable )
    // if we had used 'let mut spaces' it would have gave error because even if variable is immutable, we can't change its type
    //also to note by assigning new variable ,we didn't changed its value , but it is literally new variable so if we don't specify it be mut
    // it will still remain immutable 
}
