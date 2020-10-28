use std::io;

// 1. Vytvor struct obldĺžnika.
// 2. User zadá jednu aj druhú stranu obldĺžnika
// 3. Vytvor metódy pre ten struct, ktoré vyrátajú obvod a obsah obldĺžnika
// 4. Vytvor enum, ktorý bude mať možnosti obvod a obsah
// 5. Od usera sa opýtaj, či chce vyrátať obvod alebo obsah
// 6. Matchni jeho vstup ku enum
// 7. Vypíš výsledok
struct Obdlznik {
    a: i32,
    b: i32,
}

pub fn main() {
    println!("daj rozmer a");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let strana_a: i32 = input.trim().parse().unwrap();

    println!("daj rozmer b");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let strana_b: i32 = input.trim().parse().unwrap();

    let obdlznik = Obdlznik {
        a: strana_a,
        b: strana_b,
    };

    println!("daj obvod alebo obsah (O/S");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    if input.trim() == "O" {
        println!("obdlznik ma obvod {}", obdlznik.obvod());
    }
    if input.trim() == "O" {
        println!("obdlznik ma obsah {}", obdlznik.obsah());
    }   
    }
    // println!("strana a je {} strana b je {}", obdlznik.a, obdlznik.b);
    // println!("obdlznik ma obsah {}", obdlznik.obsah());

impl Obdlznik {
    fn obsah(&self) -> i32 {
        self.a * self.b
    }

    fn obvod(&self) -> i32 {
        2 * (self.a + self.b)
    }
}

// let join3r: Adresa = Adresa {
//     //      ulica: "Henckovce".into(),
//     //      cislo_ulice: 32,

// }
