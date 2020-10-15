use std::io;

pub fn main() {
    let mut cisla: Vec<i32> = Vec::new();
    loop {
        println!("daj cislo");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input.trim() == "x" {
            break;
        } else {
            let input: i32 = input.trim().parse().unwrap();
            cisla.push(input + 1);
        }
    }
    println!("nejde to");
}
