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
