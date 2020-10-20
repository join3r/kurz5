// User zadá čísla a ukončí zadávanie tak, že miesto čísla zadá písmeno 'x'
// Zapíš čísla do súboru do Upload/subor.csv
// Príklad:
// 1
// 2
// 3
// 5
// 8
// 100
// x
// Program zapíše do súboru, vždy o jedno číslo väčšie:
// 2,3,4,6,9,101

use std::{fs::File, io};
use std::io::Write;
use std::fs::OpenOptions;
pub fn main() {
    println!("zadavaj cisla");
    let mut cisla: Vec<i32> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "x" {
            break;
        } else {
            let input_cislo:i32 = input.trim().parse().unwrap();
            cisla.push(input_cislo + 1);
        }
    };
    // println!("{:?}", cisla); // pouzit Debug trait sa nerata, chcem vypisat vsetky veci oddelene ciarkou ako je v priklade

    for i in cisla {
        print!("{}, ", i); // nič nebude vidno na obrazovke po výpise => https://doc.rust-lang.org/std/macro.print.html
    };

    // fs::write("subor.csv", numbers);

}
