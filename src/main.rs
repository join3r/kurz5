use std::io;

mod bachi3;
mod evie3;
mod firestorm3;
mod hydrant3;
mod osiris3;

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

// bachi, evie, firestorm, fusekla, hydrant

enum Mena {
    Bachi,
    Evie,
    Firestorm,
    Fusekla,
    Hydrant,
}

impl LepsiString {
    fn parse_to_enum(&self) -> Mena {
        match self.inner.as_str() {
            "bachi" => Mena::Bachi,
            "evie" => Mena::Evie,
            "firestorm" => Mena::Firestorm,
            "fusekla" => Mena::Fusekla,
            "hydrant" => Mena::Hydrant,
            _ => panic!("Zly input"),
        }
    }
}

struct LepsiString {
    inner: String,
}

impl Mena {
    fn show_all() {
        println!("{}", Mena::Bachi.show());
        println!("{}", Mena::Evie.show());
        println!("{}", Mena::Firestorm.show());
        println!("{}", Mena::Fusekla.show());
        println!("{}", Mena::Hydrant.show());
    }

    fn show(&self) -> &str {
        match self {
            Mena::Bachi => "Bachi",
            Mena::Evie => "Evie",
            Mena::Firestorm => "Firestorm",
            Mena::Fusekla => "Fusekla",
            Mena::Hydrant => "Hydrant",
        }
    }
}

fn main() {
    Mena::show_all();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = LepsiString {
        inner: input.trim().to_lowercase(),
    };
    match input.parse_to_enum() {
        Mena::Bachi => bachi3::main(),
        Mena::Evie => evie3::main(),
        Mena::Firestorm => firestorm3::main(),
        Mena::Fusekla => osiris3::main(),
        Mena::Hydrant => hydrant3::main(),
    }
}

// while, loop
// struct, enum
// result
