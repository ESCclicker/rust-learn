// Concise Control Flow with if let and let...else
fn main() {
     let config_max = Some(3u8);
     match config_max {
             Some(mx) => println!("The maximum is configured to be {mx}"),
             _ => (),
     }
    
    //shorter way of doing this ................but u use the exhaustive checking which match do
    if let Some(mx) = config_max {
        println!("The maximum is configured to be {mx}");
    }
    
     
}

