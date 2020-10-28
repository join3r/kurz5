// 1. Vytvor struct obldĺžnika.
// 2. User zadá jednu aj druhú stranu obldĺžnika
// 3. Vytvor metódy pre ten struct, ktoré vyrátajú obvod a obsah obldĺžnika
// 4. Vytvor enum, ktorý bude mať možnosti obvod a obsah
// 5. Od usera sa opýtaj, či chce vyrátať obvod alebo obsah
// 6. Matchni jeho vstup ku enum
// 7. Vypíš výsledok

use std::io;
struct Obdlznik {
    strana_a: u32,
    strana_b: u32,
}

pub fn main() {
    println!("Zadaj rozmer strany a:");
    let mut input_usera = String::new();
    io::stdin().read_line(&mut input_usera);
    let strana_a_cislom: u32 = input_usera.trim().parse().unwrap();

    println!("Zadaj rozmer strany b:");
    let mut input_usera = String::new();
    io::stdin().read_line(&mut input_usera);
    let strana_b_cislom: u32 = input_usera.trim().parse().unwrap();

    let obdlznicek: Obdlznik = Obdlznik {
        strana_a: strana_a_cislom,
        strana_b: strana_b_cislom,
    };
    println!(
        "Rozmery obdlznika su {} a {}",
        obdlznicek.strana_a, obdlznicek.strana_b
    );
}

impl Obdlznik {
    fn vyrataj_obsah(self) -> u32 {
        self.strana_a * self.strana_b
    }
    fn vyrataj_obvod(self) -> u32 {
        2 * (self.strana_a + self.strana_b)
    }
}

println!("Obsah obdlznika je {} a obvod je {}", )

//         println!("{} {}", self.ulica, self.cislo_ulice);
//     }
// }

// assert_eq!(join3r.cislo_ulice, 32);
