use std::io;

pub fn main() {
    println!("Zadaj den: ");
    let mut den = String::new();
    io::stdin().read_line(&mut den);
    den = den.trim().into();

    let mut index: usize;
    loop {
      println!("Zadaj mesiac: ");
      let mut mesiac = String::new();
      io::stdin().read_line(&mut mesiac);
      index = mesiac.trim().parse().unwrap();
      if index > 0 && index < 13 {
        break;
      }
    }

    println!("Zadaj rok: ");
    let mut rok = String::new();
    io::stdin().read_line(&mut rok);
    rok = rok.trim().into();

    let mesiace = ["Januar", "Februar", "Marec", "April", "Maj", "Jun", "Jul", "August", "September", "Oktober", "November", "December"];

    println!("Typecek zadal: {} {} {}", den, mesiace[index-1], rok);
}