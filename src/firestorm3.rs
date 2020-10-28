
// 1. Vytvor struct obldĺžnika.
// 2. User zadá jednu aj druhú stranu obldĺžnika
// 3. Vytvor metódy pre ten struct, ktoré vyrátajú obvod a obsah obldĺžnika
// 4. Vytvor enum, ktorý bude mať možnosti obvod a obsah
// 5. Od usera sa opýtaj, či chce vyrátať obvod alebo obsah
// 6. Matchni jeho vstup ku enum
// 7. Vypíš výsledok

#[derive(Copy, Clone)]
struct Rect {
    a: i32,
    b: i32,
}

impl Rect {
    fn count_perimeter(&self) -> i32 {
        2 * (self.a + self.b)
    }
    fn count_area(&self) -> i32 {
        self.a * self.b
    }
}

enum Op {
    Perimeter,
    Area,
}

pub fn main() {

    let mut side_a = String::new();
    let mut side_b = String::new();
    let mut input = String::new();
    
    
    println!("Input side a : ");
    std::io::stdin().read_line(&mut side_a);
    
    println!("Input side a : ");
    std::io::stdin().read_line(&mut side_b);


    let rect: Rect = Rect {
        a: side_a.trim().parse().unwrap(),
        b: side_b.trim().parse().unwrap(),
    };

   
    println!("What would you like to count ?");
    println!("a - area,  p - perimeter ");
    std::io::stdin().read_line(&mut input);
    let op = input.trim();
    // let obvod = rect.clone().count_perimeter();

    println!("You have entered a: {} b: {}", rect.a, rect.b );
    println!("You have entered operation: {} ", op);

   

    match op {

    }


    if input.trim() == "a" {
        let operation = Op::Area;
    }
    if input.trim() == "p" {
        let operation = Op::Perimeter;
    }





  


    







}