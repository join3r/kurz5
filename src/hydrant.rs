use std::io;

pub fn main() {
    print!("Zadaj den: ");
    let mut den = String::new();
    io::stdin().read_line(&mut den);

    print!("Zadaj mesiac: ");
    let mut mesiac = String::new();
    io::stdin().read_line(&mut mesiac);
    let mesiacI:u8 = u8::from(mesiac);

    print!("Zadaj rok: ");
    let mut rok = String::new();
    io::stdin().read_line(&mut rok);

    if mesiacI == 1 {
      let mesiacX = "Januar";
    } else if mesiacI == 2 {
      let mesiacX = "Februar";
    } else if mesiacI == 3 {
      let mesiacX = "Marec";
    } else if mesiacI == 4 {
      let mesiacX = "April";
    } else if mesiacI == 5 {
      let mesiacX = "Maj";
    } else if mesiacI == 6 {
      let mesiacX = "Jun";
    } else if mesiacI == 7 {
      let mesiacX = "Jul";
    } else if mesiacI == 8 {
      let mesiacX = "August";
    } else if mesiacI == 9 {
      let mesiacX = "September";
    } else if mesiacI == 10 {
      let mesiacX = "Oktober";
    } else if mesiacI == 11 {
      let mesiacX = "November";
    } else if mesiacI == 12 {
      let mesiacX = "December";
    } else {
      let mesiacX = "GLUPOCINA!";
    }

    println!("Typecek zadal: {} {} {}", den, mesiacX, rok);
}