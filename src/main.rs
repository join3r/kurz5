use std::io;

mod osiris2;
mod evie2;
mod hydrant2;
mod firestorm2;
mod bachi2;

// let mut cisla: Vec<i32> = Vec::new(); <-- prazdny vector
// cisla.push(1);
// cisla.push(2);
// assert_eq!(cisla[0], 1);
// assert_eq!(cislo[1], 2);

// let cisla: Vec<i32> = vec![1, 2, "Bimbo"]; <-- Neslo
// let mena: Vec<&str> = vec!["Jano", "Pista", "Adolf"]

// let mesiace = ["Januar", "Februar", "Marec", "April", "Maj", "Jun", "Jul", "August", "September", "Oktober", "November", "December"];
// let index: usize = 10;
// assert_eq!(mesiace[index], "Januar");

// let slovo: String = "10";
// let cislo: i32 = slovo.parse().unwrap();

// let slovo = "slovo\n";
// assert_eq!(slovo.trim(), "slovo");

fn main() {
    println!("osiris");
    println!("evie");
    println!("hydrant");
    println!("firestorm");
    println!("bachi");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "osiris\n" {
            osiris2::main();
            break;
        }
        if input == "evie\n" {
            evie2::main();
            break;
        }
        if input == "hydrant\n" {
            hydrant2::main();
            break;
        }
        if input == "firestorm\n" {
            firestorm2::main();
            break;
        }
        if input == "bachi\n" {
            bachi2::main();
            break;
        }
        println!("Zle meno ďula");
    }
}

// while, loop
// struct, enum
// result