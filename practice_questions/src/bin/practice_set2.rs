use std::{collections::HashMap, vec};




fn main() {
    let mut vec = vec![3,4,8,5,2,3,4,7,8];
    find_median_mode(&mut vec);
    convert();
}


// Given a list of integers, use a vector and return the median (when sorted, the value in the 
// middle position) and mode (the value that occurs most often; a hash map will be helpful here) 
// of the list.

fn find_median_mode(vec: &mut Vec<i32>) {
    
    ascend(vec);
    println!("{vec:?}");
    if vec.len() % 2 == 0 {
            let div = vec.len() / 2 ;
            let median = vec[div-1] + vec[div];
            println!("The median is {}", median);
    } else {
      let div = vec.len() / 2 ;
      let median = vec[div];
      println!("The median is {}",median);
    }

    let mut map = HashMap::new();
      for i in 0..vec.len() {
        let count = map.entry(vec[i]).or_insert(0);
        *count += 1 ;
      }
    println!("{map:?}");
    let mut max_value = 0;
    let mut highest_occrance = 0 ;
    for (key,value) in &map {
         if value > &max_value {
                   max_value = *value;
                   highest_occrance = *key ; 
            }
    }
    println!("The mode is {}",highest_occrance);


}


fn ascend(data: &mut Vec<i32> ) { 
           for x in 0..data.len() {
             for i in 0..data.len()-x-1{
              if data[i] > data[i+1] {
                data.swap(i,i+1);
               }

             }
           }
}      


// Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

fn convert() {
      let  string = "hello i am honest person".to_string();
      let  mut result = String::new();
      // let mut map = HashMap::new();
      for word in string.split_whitespace() {
          //  let ele = map.entry(word).or_insert(0);
          let mut new_word = word.to_string();
          if matches!(
           word.chars().next().unwrap().to_ascii_lowercase(),
               'a' | 'e'| 'i' | 'o' | 'u' 
          )   {
            new_word.push_str("-hay");
          }
          
          else { 
              let mut chars = new_word.chars();
              let first = chars.next().unwrap();
              let rest: String = chars.collect();
              new_word = format!("{}-{}ay",rest,first);
          }
          result.push_str(&new_word);
          result.push(' ');
      }
      println!("{}",result);

} 