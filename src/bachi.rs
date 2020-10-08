use std::io::{self, Write};
pub fn main() {
 let mesiace = ["Januar", "Februar", "Marec","april","Maj","Jun","Jul","August","September", "Oktober", "November", "December"];
// assert_eq!(mesiace[0], "Januar");


    // println!("Zadaj den tu: ");
    // println!("Zadaj mesiac tu: ");
    // println!("Zadaj rok tu: ");
    let moje_a: String = get_input_from_user("Zadaj den ");
  let moje_a: i32 = moje_a.parse().unwrap();



  
  let moje_b: String = get_input_from_user("Zada mesiac ");
  let moje_b: usize = moje_b.parse().unwrap();
  let mesiac:&str= mesiace[moje_b -1]; 




  let moje_c: String = get_input_from_user("Zadaj rok ");
  let moje_c: i32 = moje_c.parse().unwrap();
  
  println!("Narodil si sa : {} {} {} ",moje_a, mesiac,moje_c );

  
    // let mut den = String::new();
    // io::stdin().read_line(&mut den);

    // let mut mesiac = String::new();
    // io::stdin().read_line(&mut mesiac);

    // let mut rok = String::new();
    // io::stdin().read_line(&mut rok);

  
       



}


pub fn get_input_from_user<T, U>(msg: U) -> T
where
    T: std::str::FromStr,
    U: std::fmt::Display,
{
    loop {
        let mut input = String::new();
        print!("{}", msg);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(res) => return res,
            Err(_) => println!("Invalid input"),
        };
    }
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