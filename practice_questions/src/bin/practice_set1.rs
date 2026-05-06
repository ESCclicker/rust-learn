// Took AI help for Fibonacci and Christmas Carol logic 
// did fib last part logic with it and carol rev 


use std::io;
fn main() {
    loop {
    println!("what operation you want to do ?");
    println!("1.Convert Temp between farenheit and celsius\n2.Generate the nth fibonacci number\n3.get lyrics of christmas carol\nquit program");
    println!("please enter your choice 1,2,3,q");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed to read main fn choice");
    let choice = choice.trim() ;
    if choice == "1" { 
        converter();
    } else if choice == "2"{
        fib();
    } else if choice == "3" {
        song();
    } else if choice == "q" {
        break;
    } else {
        println!("please specify choice correctly");
    }
}
    println!("hope you loved our service 🦀")
}
// 1. Convert temperatures between Fahrenheit and Celsius.
// C = (F-32)*(5/9)
// F = (C*(9/5))+32
fn converter() {
    loop {
        println!("select from which to which you want to convert");
        println!("1.from farenheit to celsius\n2. from celsius to farenheit\nto quiet enter 'q'");
        println!("enter your choice (eg 1 or 2 or q)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read the line");
        let choice = choice.trim(); //who tf will write it ? 😭😭 (remember read_line is returning value+\n so we have to trim that line)
        if choice == "1" {
            println!("enter the temp in farenheit(only number)");
            let mut f_temp = String::new();
            io::stdin()
                .read_line(&mut f_temp)
                .expect("failed to read f_temp");
            let f_temp: f64 = f_temp.trim().parse().expect("failed to convert f_temp");
            let c_temp_re = ((f_temp - 32.0) * 5.0) / 9.0;
            println!(
                "Temp Given by you in Farenheit: {f_temp}F\ncoverted temp in celsius:{c_temp_re}C"
            );
        } else if choice == "2" {
            println!("enter the temp in celsius(only number)");
            let mut c_temp = String::new();
            io::stdin()
                .read_line(&mut c_temp)
                .expect("failed to read c_temp");
            let c_temp: f64 = c_temp.trim().parse().expect("failed to convert c_temp");
            let f_temp_re = (c_temp * (9.0 / 5.0)) + 32.0;
            println!(
                "Temp Given by you in Celsius: {c_temp}C\nconverted temp in farenheit: {f_temp_re}F"
            );
        } else if choice == "q" {
            break;
        } else {
            println!("can't understand your choice\n(please specify choice correctly 1 or 2 or q )")
        }
    }
}
//2.Generate the  n^th Fibonacci number.
fn fib() {
    println!("tell the nth fibonacci number you want to generate (limit upto 100)");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to read n");
    let n: u32 = n.trim().parse().expect("failed to convert n");
    if n > 100 {
        println!("you input was too big");
    } else if n == 0 {
        println!("0th fib no. is 0");
    } else {
        let mut prev = 0u128; //u128 to handle 100
        let mut current = 1u128;

        for _ in 1..n {
            let next = prev + current;
            prev = current;
            current = next;
        }

        println!("{n}th fib no. is {current}");
    }
}

//3.Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn song() {
    let days = [
        "first", "second", "Third", "fourth", "fifth", "Sixth", "Seventh", "Eight", "Ninth",
        "Tenth", "Eleventh", "Twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "four calling hens",
        "five gold rings",
        "six geese a laying",
        "seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve Drummers drumming",
    ];
    for day in 0..12 {
        println!("On the {} day of christmas my true love sent me", days[day]);
        for gift in (0..=day).rev() {
            if gift == 0 && day > 0 {
                println!("And {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
    }
}
