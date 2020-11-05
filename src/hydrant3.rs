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

  let mut input: String = String::new();

  println!("Poratam obsah, ci obvod?");
  io::stdin().read_line(&mut input).unwrap();
  let o = MyString {
    inner: input,
  };

  // let obvod = strany.obvod();
  // let obsah = strany.obsah(); // <-- strany moved



  match o.parse_to_enum() {
    Porataj::Obsah => println!("Obsah: {}", strany.obsah()),
    Porataj::Obvod => println!("Obsah: {}", strany.obvod()),
  }

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
  Obsah,
}

struct MyString {
  inner: String
}

impl MyString {
  fn parse_to_enum(&self) -> Porataj {
    match self.inner.trim().to_lowercase().as_str() { // trim() "obvod\n"
      "obvod" => Porataj::Obvod,
      "obsah" => Porataj::Obsah,
    }
  }
}

#[derive(Copy, Clone)]
struct Obdlznika {
  a: u32,
  b: u32,
}

// stack: ja_som_string_ale_nie_som_tu_som_na_heape_tam_adresa_0x032148102fd
// stack: ja_som_string_2_ale_nie_som_tu_som_na_heape_tam_adresa_0x032148102fd
// 0x032148102fd heap: "Janko Mrkvicka je v skutocnosti Osiris"