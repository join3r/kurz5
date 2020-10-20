use std::fs;
use std::io::stdin;

pub fn main() {
    let mut numbers: Vec<String> = Vec::new();
    let filename = "subor.csv";

    loop {
        let mut input = String::new();
        println!("Input number or x to end: ");
        std::io::stdin().read_line(&mut input);

        if input.trim() != "x" {
            let number: i32 = input.trim().parse().unwrap();
            numbers.push((number + 1).to_string());
        } else {
            break;
        }
    }

    let line = numbers.join(",");
    fs::write(filename, line);
}
