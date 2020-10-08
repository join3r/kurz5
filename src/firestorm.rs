use std::io;

pub fn main() {

// Vyžiadaj deň, mesiac a rok od usera (ako čísla)
// Vypíš meno mesiaca, ktoré zadal user:
// Napríklad. User zadal
// 10
// 5
// 1982
// Výstup: 10 Máj 1982


// struct Month
let months = ["Januar", "Februar", "Marec","april","Maj","Jun","Jul","August","September", "Oktober", "November", "December"];


// assert_eq!(mesiace[0], "Januar");

    let mut day = String::new();
    let mut month = String::new();
    let mut year = String::new();

    println!("Zadaj den: ");
    io::stdin().read_line(&mut day);
    let day_n: i32 = day.trim().parse().unwrap(); // Result<_,_>

    
    println!("Zadaj mesiac: ");
    io::stdin().read_line(&mut month);
    let month_n: usize = month.trim().parse().unwrap();

    
    println!("Zadaj rok: ");
    io::stdin().read_line(&mut year);
    let year_n: i32 = year.trim().parse().unwrap();
    
    println!("Vystup: {} {} {}", day_n,  if month_n <= 12 || month_n >= 1 { months[month_n - 1] } else { "Neznamy mesiac" }, year_n);
   
    
}


