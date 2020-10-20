use std::io;
use std::fs::File;
use std::io::Write;

pub fn main()  {
    let mut cisla: Vec<i32> = Vec::new();
    let mut cisla_string: String;

    println!("Zadaj cisla, pre ukoncenie vstupu zadaj x");
    funkcia(&mut cisla);
    for f in cisla {
        print!("{}, ", f);
        // cisla_string.push_str(&f.to_string());
        // cisla_string.push_str(",");
    }
    // cisla_string = cisla.concat();
    // println!();
    // let mut file = File::create("foo.txt").unwrap();
    // file.write_all(cisla_string.as_bytes()).unwrap(); // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/create.html

}

fn funkcia(cisla: &mut Vec<i32>) {
    let mut input_i32: i32;
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "x" {
            break;
        } else {
            input_i32 = input.trim().parse().unwrap();
            cisla.push(input_i32 + 1);
        }
    }
}
