use std::io;

mod osiris3;
mod evie3;
mod hydrant3;
mod firestorm3;
mod bachi3;

// struct Adresa {
//      ulica: String,
//      cislo_ulice: i32,
// }

// impl Adresa {
//     fn vypis_adresu(self) {
//         println!("{} {}", self.ulica, self.cislo_ulice);
//     }
// }

// let join3r: Adresa = Adresa {
    //      ulica: "Henckovce".into(),
    //      cislo_ulice: 32,
    // };
// join3r.vypis_adresu();
// assert_eq!(join3r.cislo_ulice, 32);


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
            osiris3::main();
            break;
        }
        if input == "evie\n" {
            evie3::main();
            break;
        }
        if input == "hydrant\n" {
            hydrant3::main();
            break;
        }
        if input == "firestorm\n" {
            firestorm3::main();
            break;
        }
        if input == "bachi\n" {
            bachi3::main();
            break;
        }
        println!("Zle meno ďula");
    }
}

// while, loop
// struct, enum
// result