//Vyžiadaj deň, mesiac a rok od usera (ako čísla)
//Vypíš meno mesiaca, ktoré zadal user:
//Napríklad. User zadal
//10
//5
//1982
//Výstup: 10 Máj 1982

use std::io;
pub fn main() {
    println!("Zadaj den, napriklad '10':");
    let mut den = String::new();
    io::stdin().read_line(&mut den);

    println!("Zadaj mesiac, napriklad '5':");
    let mut mesiac = String::new();
    io::stdin().read_line(&mut mesiac);

    println!("Zadaj rok:");
    let mut rok = String::new();
    io::stdin().read_line(&mut rok);

    println!{"{},{},{}", den, mesiac, rok};
}

// mod osiris;
// mod evie;
// mod hydrant;
// mod firestorm;
// mod bachi;

// fn main() {
//     println!("osiris");
//     println!("evie");
//     println!("hydrant");
//     println!("firestorm");
//     println!("bachi");

//     loop {
//         let mut input = String::new();
//         io::stdin().read_line(&mut input);

//         if input == "osiris\n" {
//             osiris::main();
//             break;
//         }
//         if input == "evie\n" {
//             evie::main();
//             break;
//         }
//         if input == "hydrant\n" {
//             hydrant::main();
//             break;
//         }
//         if input == "firestorm\n" {
//             firestorm::main();
//             break;
//         }
//         if input == "bachi\n" {
//             bachi::main();
//             break;
//         }
//         println!("Zle meno ďula");
//     }
// }

// // while, loop
// // struct, enum
// // result
