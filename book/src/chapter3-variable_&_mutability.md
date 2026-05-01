# Variables


From Start Variable are immutable in Rust
So when we run this code 
```rust
fn main() {

    let x= 5;

    println!("the value of x is : {x}");

    x = 6 ;

    println!("The value of x is : {x}");

}
```
it will return an output error like this when we will run it :
```bash
PS D:\rustproj\variables> cargo run 
   Compiling variables v0.1.0 (D:\rustproj\variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src\main.rs:4:5
  |
2 |     let x= 5;
  |         - first assignment to `x`
3 |     println!("the value of x is : {x}");
4 |     x = 6 ;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x= 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error
PS D:\rustproj\variables> 
```

now to make the variable mutable add ``mut`` here :
```rust
  let mut x = 5 ;
```

and it will give a output like :
```bash
PS D:\rustproj\variables> cargo run
   Compiling variables v0.1.0 (D:\rustproj\variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target\debug\variables.exe`
the value of x is : 5
The value of x is : 6
PS D:\rustproj\variables> 

```
so by adding ==``mut``== we can decide if we want to make our variable mutable or not .





# Shadowing

Shadowing a variable means , we can declare a new variable with the same name as previous variable . The Second variable **`Overshadows`** the first Variable until it gets overshadowed by another new variable with the same name or the scope ends 
we can shadow a variable like this 

```rust
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
```

> [!question]- what happened in code ?
> so firstly we declared a mutable variable x ( `let mut x = 5;`) then we changed the value of mutable variable x to `x = x -1 ;` i.e. 4 , now we overshadowed the `let mut x = 5 ;`  by again declaring the x using `let x = x + 1;` , which added +1 to value of x and also made it immutable , then in a inner scope we overshadowed (made it mutable again ) the second variable and then  printed its value and printed its value and then changed it and printed again but when we came out of inner scope the x again used the value of second variable and become immutable again 


and it will give an output like this :


>[!done]- output
>```bash
PS D:\rustproj\variables> cargo run
   Compiling variables v0.1.0 (D:\rustproj\variables)
>  Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
 >    Running `target\debug\variables.exe`
>the value of x in the inner scope is : 10
>4
>the value of x is : 5
>```
>
>>



# Shadowing vs mut

>[!question]- mut VS shadowing
>
>Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have completed.
The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

```rust
fn main(){

    let mut spaces = "    ";

    spaces = spaces.len();

    println!("spaces to give are {spaces}")

  

}
```

>[!error]- Above code will give error ?
>The above code will give an error like this :-
>```bash
>
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`
>
>For more information about this error, try `rustc --explain E0308`.
>error: could not compile `variables` (bin "variables") due to 1 previous error
>```
>
>because we tried to change type even if the variable is mutable , we can't change it's type Now that's where **`shadowing`** come at work , we can simply assign a type to variable by using **`let`** 


Fixed code :
```rust
fn main(){

    let spaces = "    ";

    let spaces = spaces.len();

    println!("spaces to give are {spaces}")

  

}
```
it fixed the problem because here we are effectively creating a new variable so it doesn't give error 
also it is still immutable 

>[!done]- Output will like this 
>```bash
>PS D:\rustproj\variables> cargo run --bin shadowing
   Compiling variables v0.1.0 (D:\rustproj\variables)
 >   Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.55s
  >   Running `target\debug\shadowing.exe`
>spaces to give are 4
>PS D:\rustproj\variables>
>```
>