use std::io;

pub fn main() {
    let mut den: i32;
    loop {
        println!("Zadaj den (1-31)");
        let mut den_slovom = String::new();
        io::stdin().read_line(&mut den_slovom);

        den = den_slovom.trim().parse().unwrap();
        if den < 1 {
            println!("Vedze nie takto!");
        };
        if den > 31 {
            println!("Vedze nie takto!");
        } else {
            break;
        }
    }

    let mut mesiac_cislom: usize;
    loop {
        println!("Zadaj mesiac (1-12)");

        let mut mesiac_slovom = String::new();
        io::stdin().read_line(&mut mesiac_slovom);
        mesiac_cislom = mesiac_slovom.trim().parse().unwrap();
        if mesiac_cislom < 1 {
            println!("Vedze nie takto!");
        };
        if mesiac_cislom > 12 {
            println!("Vedze nie takto!");
        } else {
            break;
        };
    }

    let mut rok: i32;
    loop {
        println!("Zadaj rok (1-9999)");

        let mut rok_slovom = String::new();
        io::stdin().read_line(&mut rok_slovom);

        rok = rok_slovom.trim().parse().unwrap();
        if rok < 1 {
            println!("Vedze nie takto!");
        };
        if rok > 9999 {
            println!("Vedze nie takto!");
        } else {
            rok;
            break;
        };
    }

    let mesiace = [
        "Januar",
        "Februar",
        "Marec",
        "April",
        "Maj",
        "Jun",
        "Jul",
        "August",
        "September",
        "Oktober",
        "November",
        "December",
    ];
    // assert_eq!(mesiace[0], "Januar");

    println!("{} {} {}", den, mesiace[mesiac_cislom - 1], rok);
}

// Vyžiadaj číslo od usera
