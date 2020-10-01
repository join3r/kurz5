use std::io;

pub fn main() {

    println!("Zadaj den");
    println!("Zadaj mesiac");
    println!("Zadaj rok");
    
    let mut den = String::new();
    io::stdin().read_line(&mut den);

    let mut mesiac = String::new();
    io::stdin().read_line(&mut mesiac);

    let mut rok = String::new();
    io::stdin().read_line(&mut rok);

  
       



}
//     Vyžiadaj deň, mesiac a rok od usera (ako čísla)
// Vypíš meno mesiaca, ktoré zadal user:
// Napríklad. User zadal
// 10
// 5
// 1982
// Výstup: 10 Máj 1982
//     fn main() {
//         println!("Zadaj den mesiac a rok")
//         loop {
//             let mut input = String::new();
//             io::stdin().read_line(&mut input);
    
//             if input == "osiris\n" {
//                 osiris::main();
//                 break;
//             }
//             if input == "evie\n" {
//                 evie::main();
//                 break;
//             }
//             if input == "hydrant\n" {
//                 hydrant::main();
//                 break;
//             }
//             if input == "firestorm\n" {
//                 firestorm::main();
//                 break;
//             }
//             if input == "bachi\n" {
//                 bachi::main();
//                 break;
//             }
//             println!("Zle meno ďula");
//         }
//     }
    
//     // while, loop
//     // struct, enum
//     // result
// }