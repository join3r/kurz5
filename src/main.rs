use std::io;

mod osiris;
mod evie;
mod hydrant;
mod firestorm;
mod bachi;

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
        io::stdin().read_line(&mut input);

        if input == "osiris\n" {
            osiris::main();
            break;
        }
        if input == "evie\n" {
            evie::main();
            break;
        }
        if input == "hydrant\n" {
            hydrant::main();
            break;
        }
        if input == "firestorm\n" {
            firestorm::main();
            break;
        }
        if input == "bachi\n" {
            bachi::main();
            break;
        }
        println!("Zle meno ďula");
    }
}

// while, loop
// struct, enum
// result