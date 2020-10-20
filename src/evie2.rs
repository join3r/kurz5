use std::{
    fs::File,
    io::{self, Write},
};

pub fn main() {
    let mut cisla: Vec<i32> = Vec::new();
    println!("Zadaj cisla:");
    loop {
        let mut cislo = String::new();
        io::stdin().read_line(&mut cislo);
        if cislo.trim() == "x" {
            break;
        }
        let zadane_cislo: i32 = cislo.trim().parse().unwrap();
        cisla.push(zadane_cislo + 1);
    }

    // let mut ofile = File::create("subor.txt").expect("unable to create file");
    // ofile.write_all(cisla.as_bytes()).expect("unable to write");
}
