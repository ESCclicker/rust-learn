fn main() { 
        let _s1 = gives_ownership(); //gives_ownership moves its return value into s1

        let s2 = String::from("hello"); //s2 comes into scope

        let _s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, which also moves its 
                                           // return value into s3
} //here,s3 goes out of scope and is dropped, s2 was moved , so nothing happens
// s1 goes out of scope and is dropped 


fn gives_ownership() -> String {  //gives ownership will move its return value into the function that calls it 
    
    let some_string = String::from("yours"); //some_string comes into scope

    some_string //some_string is returned and moves out to the calling function 
}

//This function takes a String and returns a String
fn takes_and_gives_back(a_string : String) -> String { 
      //a_string comes into scope
      
      a_string //a_string is returned and moves out to the calling function
}



//what happened?

/*
fistly main fucntion me hamne gives_ownership ko bulaya 
then gives_ownership ne apni return value bheji main function me _s1 me 
now uske baad hamne main function me EK s2 string assign ki and then 
_s3 = takes_and_gives_back(s2) kiya 
jo takes and give back fucntion me s2 as parameter gya 
then takes_.. function ne apni return value ( jo ki wohi string thi jo usse mili thi)
usse return kardiya
and wo s3 ko milgyi 
*/



//When a variable that includes data on the heap goes out of scope,
// the value will be cleaned up by drop unless 
//ownership of the data has been moved to another variable.


//Rust does let us return multiple values using a tuple (showed in chap8_ex3.rs)