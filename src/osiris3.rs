// 1. Vytvor struct obldĺžnika.
// 2. User zadá jednu aj druhú stranu obldĺžnika
// 3. Vytvor metódy pre ten struct, ktoré vyrátajú obvod a obsah obldĺžnika
// 4. Vytvor enum, ktorý bude mať možnosti obvod a obsah
// 5. Od usera sa opýtaj, či chce vyrátať obvod alebo obsah
// 6. Matchni jeho vstup ku enum
// 7. Vypíš výsledok

// struct Adresa {
//      ulica: String,
//      cislo_ulice: i32,
// }
// let join3r: Adresa = Adresa {
//      ulica: "Henckovce".into(),
//      cislo_ulice: 32,
// }
// assert_eq!(join3r.cislo_ulice, 32);

use std::io;

#[derive(Copy, Clone)]
struct Rozmery {
    dlzka: i32,
    sirka: i32,
}

impl Rozmery {
    fn obvod(&self) -> i32 {
        2 * (self.dlzka + self.sirka)
    }
    fn obsah(&self) -> i32 {
        self.dlzka * self.sirka
    }
}

pub fn main() {
    println!("Zadaj dlzku :");
    let mut dlzka = String::new();
    io::stdin().read_line(&mut dlzka).unwrap();

    println!("Zadaj sirku :");
    let mut sirka = String::new();
    io::stdin().read_line(&mut sirka).unwrap();

    let obdlznik: Rozmery = Rozmery {
        dlzka: dlzka.trim().parse().unwrap(),
        sirka: sirka.trim().parse().unwrap(),
    };

    println!("rozmery su {} a {}", obdlznik.dlzka, obdlznik.sirka);

    println!("Obvod/obsah?");
    let mut lojzo: String = String::new();
    io::stdin().read_line(&mut lojzo).unwrap();


    // match lojzo {
    //     operacia::obsah => println!("Obsah je {}", obdlznik.obsah()),
    //     operacia::obvod => println!("Obsah je {}", obdlznik.obvod()),
    //     _ => (),
    // }

}

enum operacia {
    obvod,
    obsah,
}
