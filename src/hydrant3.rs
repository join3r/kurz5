use std::io;

pub fn main() {

  let mut a: String = String::new();
  let mut b: String = String::new();
  
  println!("Zadaj a:");
  io::stdin().read_line(&mut a).unwrap();

  println!("Zadaj b:");
  io::stdin().read_line(&mut b).unwrap();

  let strany: Obdlznika = Obdlznika {
    a: a.trim().parse().unwrap(),
    b: b.trim().parse().unwrap(),
  };

  println!("Tote strany si zadal: {} a {}", strany.a, strany.b);

  let mut o: String = String::new();

  let obvod = strany.obvod();
  let obsah = strany.obsah(); // <-- strany moved

// match o {
//   Porataj::Obsah => println!("Obsah: {}", strany.obsah()),
//   Porataj::Obvod => println!("Obsah: {}", strany.obvod()),
// }

  // match o.try_into() {
  //   Ok(porataj::obvod) => obvod(strany),
  //   Ok(porataj::obsah) => obsah(strany),
  //   _ => Err("Nem dobre"),
  // }
}

impl Obdlznika {
  fn obsah(&self) -> u32 {
    self.a*self.b
  }

  fn obvod(&self) -> u32 {
    2*(self.a+self.b)
  }
}

enum Porataj {
  Obvod,
  Obsah
}

#[derive(Copy, Clone)]
struct Obdlznika {
  a: u32,
  b: u32,
}

// stack: ja_som_string_ale_nie_som_tu_som_na_heape_tam_adresa_0x032148102fd
// stack: ja_som_string_2_ale_nie_som_tu_som_na_heape_tam_adresa_0x032148102fd
// 0x032148102fd heap: "Janko Mrkvicka je v skutocnosti Osiris"