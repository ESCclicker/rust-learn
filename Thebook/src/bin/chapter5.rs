//functions

fn another_function(){
    print!("hello world.");
}

fn main() {              //rust doesn't care about which fucntion u define first ( only care 
                        //  they are defined somewhere in a scope that can be seen by the caller
    another_function();  //also have to call that fucntion in main() to use it
    parameter(5,'m'); //pass the parameters here 
    statements();
    expression();
    let a = five(); // here five() will return value 5 
    println!("the value of x is : {a}");
    let b = plus_one(5);
    println!("The value of b is {b}");

}

//parameters
//we have to specify the type of parameters in the function signature (IMP)

fn parameter(value:i32,unit_label:char){
          println!("the measurement is :{value}{unit_label}");
}

fn statements(){  //Statements are instructions that perform some action and do not return a value.
         //creating a variable and assigning a value to it with the 'let' keyword is a statement
         let _y = 6; 
         //statements do not return value , so if we do like below 
         //let x = (let y = 6 );
         // we will get an error
}

fn expression(){ //Expressions evaluate to a resultant value.
        let y = {
            let x = 3 ;
            x+1   //not using ';' here as it will make it an statement and it will then not return value 
        };
        println!("value of y is {y}"); //evaluates to 4
}














//funtions with return values 
// example1
fn five() -> i32 {
       5 //expression whose value we want to return 
}
//example2
fn plus_one(b:i32) -> i32 {
        b+1  

/* if we write it like this b+1;  then it will give error 
because the defination of the function plus_one says that it will return an i32, but statements 
don't evaluate to a value which is expressed by '()'. Therefore nothing is returned , which contradicts
the function definition and results in an error .
*/

}