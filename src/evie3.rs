// 1. Vytvor struct obldĺžnika.
// 2. User zadá jednu aj druhú stranu obldĺžnika
// 3. Vytvor metódy pre ten struct, ktoré vyrátajú obvod a obsah obldĺžnika

// 4. Vytvor enum, ktorý bude mať možnosti obvod a obsah
// 5. Od usera sa opýtaj, či chce vyrátať obvod alebo obsah
// 6. Matchni jeho vstup ku enum
// 7. Vypíš výsledok

use std::io;
#[derive(Copy, Clone)] // typ/struct Obdlznik bude mat copy a clone traits. Za tymto nesmie byt prazdny riadok, lebo to patri k tomu structu.
struct Obdlznik {
    strana_a: u32,
    strana_b: u32,
}

// Nazov enumu (typ) ma byt s velkym pismenom a aj moznosti

// 4. Vytvor enum, ktorý bude mať možnosti obvod a obsah
enum Vypocet {
    Obvod,
    Obsah,
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

    // vytvorenie instancie obdlznicek
    // Obdlznik je nazov structu, za dvojbodkami pre strany a a b su konkretne hodnoty, ktore dostane
    let obdlznicek = Obdlznik {
        strana_a: strana_a_cislom,
        strana_b: strana_b_cislom,
    };
    println!(
        "Rozmery obdlznika su {} a {}",
        obdlznicek.strana_a, obdlznicek.strana_b
    );
    // 5. Od usera sa opýtaj, či chce vyrátať obvod alebo obsah
    println!("Chces vyratat obvod alebo obsah?");
    let mut input_usera = String::new();
    io::stdin().read_line(&mut input_usera);
}

// Obdlznik - nazov structu v ramci ktoreho sa ma implementovat metoda. Metody su obsah a obvod.
// self odkazuje k instancii obdlznicek    
impl Obdlznik {
    fn obsah(self) -> u32 {
        self.strana_a * self.strana_b
    }
    fn obvod(self) -> u32 {
        2 * (self.strana_a + self.strana_b)
    }
}



// 6. Matchni jeho vstup ku enum (definicia, ma byt mimo mainu)
impl Vypocet {
    fn na_enum(&self) -> Vypocet {
        match self.inner.as_str() {
            "obsah" => Vypocet::Obvod,
            "obvod" => Vypocet::Obsah,
            _ => panic!("Zly input"),
        }
    }
}
// 7. Vypíš výsledok


// Je rozdiel spustanie a je rozdiel definicia. Toto je spustanie a preto nemozem zadefinovat typ za dvojbodkami.
//co_vyratat(&input_usera, obdlznicek);




//Toto je len definicia funkcie, preto bude mimo mainu
// => znamena "urob"

//fn co_vyratat(input:&str, obdlznicek:Obdlznik ) {
    //match input.trim() {
        //"obsah" => println!("Obsah je: {}", obdlznicek.obsah()),
        //"obvod" => println!("Obvod je: {}", obdlznicek.obvod()),
        //_ => panic!("Zly input"),
    //}
    
    //}

