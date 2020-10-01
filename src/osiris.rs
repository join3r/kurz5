use std::io;

pub fn main() {
    println!("Zadaj den (1-31)");

    let mut den = String::new();
    io::stdin().read_line(&mut den);

    let den: i32 = den.parse().unwrap();

    loop {
     if den <1 {
        println!("Vedze nie takto!");
    };
    if den >31 {
        println!("Vedze nie takto!");     
    } else {
        break;
    }
};

    println!("Zadaj mesiac (1-12)");

    let mut mesiac = String::new();
    let mesiac: i32 = mesiac.parse().unwrap();

    loop {
        if mesiac < 1 {
           println!("Vedze nie takto!");
       };
       if mesiac > 12 {
           println!("Vedze nie takto!");      
       } else {
           break;
       };
    };
    println!("Zadaj rok (1-9999)");

    let mut rok = String::new();
    io::stdin().read_line(&mut rok);

    let rok: i32 = rok.parse().unwrap();
    
    loop {
        if rok <1 {
           println!("Vedze nie takto!");
       };
       if rok >9999 {
           println!("Vedze nie takto!");      
       } else {
           break;
       };
    };

    if mesiac == 1 {
        let mesiac = "Januar";
    };
    if mesiac == 2 {
        let mesiac = "Februar";
    };    
    if mesiac == 3 {
        let mesiac = "Marec";
    };    
    if mesiac == 4 {
        let mesiac = "April";
    };    
    if mesiac == 5 {
        let mesiac = "Maj";
    };    
    if mesiac == 1 {
        let mesiac = "Jun";
    };    
    if mesiac == 1 {
        let mesiac = "Jul";
    };    
    if mesiac == 1 {
        let mesiac = "August";
    };    
    if mesiac == 1 {
        let mesiac = "September";
    };
    if mesiac == 1 {
        let mesiac = "Oktober";
    };
    if mesiac == 1 {
        let mesiac = "November";
    };
    if mesiac == 1 {
        let mesiac = "December";
    };

println!("{} {} {}", den, mesiac, rok);    
}


// Vyžiadaj číslo od usera