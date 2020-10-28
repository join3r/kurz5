use std::{fs::File, io::{self, Write}};

pub fn main() {

  let mut cisla: Vec<String> = Vec::new();
  let mut input_i32: i32;

  println!("Zadaj cisla, pre ukoncenie vstupu zadaj x");

  loop {

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "x" {
      break;
    } else {
      input_i32 = input.trim().parse().unwrap();
      cisla.push((input_i32 + 1).to_string());
    }

  }

  println!("{}", cisla.join(","));

  let mut file = File::create("foo.txt").unwrap();
  file.write_all(cisla.join(",").as_bytes()).unwrap();
}