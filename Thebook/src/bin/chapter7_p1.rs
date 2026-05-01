//control flow

fn main(){
      if_basic();
      if_comparision();
}

//if expressions we provide condition and then state
//->If this condition is met, run this block of code. 
//If the condition is not met, do not run this block of code.

fn if_basic(){
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
}
/*fn if_use() {                                  this code will give error ?
                                                as the condition in this code must be a 
    let number = 3;                              bool ,otherwise there will be an Error 

    if number {
        println!("number was three");
    }
} */


//for above , rather than that we can do 

fn if_comparision(){
         let number = 3 ;
         if number != 0 {
            println!("number was something other than zero");
         }

}

//handling multiple conditions with else if

fn _if_else_chain(){
         let number = 6;
         if number % 4 ==0 {
            println!("number is divisible by 4");
         } else if number % 3 == 0 {
            println!("number is divisible by 3");
         } else if number % 2 ==0 {
            println!("numbe is divisible by 2")
         } else {
            println!("number is not divisible by 4,3 or 2");
         }

}
// output of above code :-> number is divisible by 3
// but note that even though 6 is divisible by 2 also , we didn't
// see the output for it ? and nor we saw the last statement?
//as once rust finds the true statement it just execute it and didn't even check the rest 

//************   using if in a let statement    ********************

// because 'if' is an expression we can use it on the right side of a 'let' statement to assign the output to a variable 

fn _if_expression(){
    let condition = true;
    let number = if condition {5} else {6};  //condition is true so output = 5
    println!("the value of number is : {number}"); //the value of number is : 5
}
// note the values in each arms of the 'if' must be the same type , otherwise we will get an Error





