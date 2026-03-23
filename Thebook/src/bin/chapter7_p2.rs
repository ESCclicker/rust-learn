//REPETION WITH LOOPS
fn main(){
     //loop_basic();
     //_loop_value_return();
     //_loop_labels();
    // _while_loop();
     
}

/*fn loop_basic(){
    loop{
        println!("again")                infinitely print "again" untill stopped
    }                                    we can stop it using break
}
*/


//RETURNING VALUES WITH LOOPS 
fn _loop_value_return(){
        let mut counter = 0 ;
        let result = loop {
            counter +=  1 ;
            if counter == 10{
                break counter*2;
            }

        };
    println!("The result is :{result}");
}

//DISAMBIGUATING WITH LOOP LABELS
fn _loop_labels(){                                                                    
        let mut count = 0;                                                               //what is happening in this code ?
        'counting_up :loop{                                     //so firstly we assignied a mutable variable count ( =0 ) then we initiated a loop ( and labelled it counting_up)
            println!("count = {count}");                        //after that we printed the count value and then assigned a new ( mutable) variable remaining (=10) then we initiated a loop
            let mut remaining = 10;                           // and in that loop we firstly printed the value of remaining and then created condition to check remaining ==9 and if true , break the inner loop
                                                              // and then the second if condition which check if count == 2 , and if it is true then we will break the outer loop ( have to mention the label)
            loop{                                            //then we lower the remaining value by one ( remaining -= 1) now the loop will again run and this time remaining( =9) so loop starting code will print
                println!("remaining = {remaining}");         //new value of remaining( remaining = 9) and then second if statement will return true ,so that inner loop will break and then after loop there again 
                if remaining == 9 {                          // we are in outer loop where the next code is count += 1 , which will inc count value and now outer loop will run again and again remaining(=10) as we assigned it again 
                                                              //then the inner loop 
                    break;                                   // till the count reaches (=2) and when it will reach 2 the outer main loop will break and then the final statement will run 
                }
                if count == 2{                  
                    break 'counting_up;
                }
                remaining -= 1 ;

            
            count += 1;
        }
    }
        println!("End count : {count}");
}

//Streamlining Conditional Loops with while
fn _while_loop() {
    let mut number = 3;            //assigned a mutable variable with value 3 
    while number != 0 {                //created a condition with while ( to run the loop untill number is not 0)
        println!("{number}!");          // printing the number
        number -= 1;                 //decreasing one from number
    }                             //going back to top and again checking if number != 0 ( if not true then loop end)

    println!("LIFT OFFFFFFFFFF!!!!!!");
}

//Looping Through a Collection with for
