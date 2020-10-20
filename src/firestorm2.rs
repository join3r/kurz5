use std::io::stdin;
use std::fs;
use std::io::Write;                                                                                                                                                                  
use std::io::prelude::*;                                                                                                                                                             
use std::fs::File;   

pub fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    let mut numbers_str: Vec<String> = Vec::new();
    // let mut f = File::create("subor.csv")?; 
    
    loop {
        let mut input = String::new();
        println!("Input number or x to end: ");
        std::io::stdin().read_line(&mut input);

       
        if input.trim() != "x" {
            let number: i32 = input.trim().parse().unwrap(); 
            numbers.push(number + 1);
            // numbers_str = numbers.into(String);
              
        } else {
            break;
        }

        

    }

    
    
    // fs::write("subor.csv", numbers);


}
