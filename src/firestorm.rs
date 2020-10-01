use std::io;

pub fn main() {

// Vyžiadaj deň, mesiac a rok od usera (ako čísla)
// Vypíš meno mesiaca, ktoré zadal user:
// Napríklad. User zadal
// 10
// 5
// 1982
// Výstup: 10 Máj 1982
    let mut day = String::new();
    let mut month = String::new();
    let mut year = String::new();

    println!("Zadaj den");
    io::stdin().read_line(&mut day);
    let mut day_n: i32 = day.parse().unwrap();
    
    println!("Zadaj mesiac");
    io::stdin().read_line(&mut month);
    let mut month_n: i32 = month.parse().unwrap();
    
    println!("Zadaj rok");
    io::stdin().read_line(&mut year);
    let mut year: i32 = day.parse().unwrap();
    
    println!("User zadal {} {} {}", day, month, year);
    
}


