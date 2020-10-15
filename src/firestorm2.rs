use std::io::stdin;

pub fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    
    
    loop {
        let mut input = String::new();
        println!("Input number or x to end: ");
        std::io::stdin().read_line(&mut input);

        if input.trim() != "x" {
            let number: i32 = input.trim().parse().unwrap(); 
            numbers.push(number);
        } else {
            break;
        }
    }
}
