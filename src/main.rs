use std::io;

mod osiris;
mod evie;
mod hydrant;
mod firestorm;
mod bachi;

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