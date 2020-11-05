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
    fn count_perimeter(&mut self) -> i32 {
        2 * (self.a + self.b)
    }
    fn count_area(&mut self) -> i32 {
        self.a * self.b
    }
    fn print_help() {
        println!("Possible methods are  to count perimeter - p, count area - a");
    }
}

enum Op {
    Perimeter,
    Area,
    Unknown,
}

pub fn main() {

    let mut side_a = String::new();
    let mut side_b = String::new();
    let mut input = String::new();
    
    
    println!("Input side a : ");
    std::io::stdin().read_line(&mut side_a);
    
    println!("Input side a : ");
    std::io::stdin().read_line(&mut side_b);


    let mut user_rect: Rect = Rect {
        a: side_a.trim().parse().unwrap(),
        b: side_b.trim().parse().unwrap(),
    };

   
    println!("What would you like to count ?");
    println!("a - area,  p - perimeter ");
    std::io::stdin().read_line(&mut input);
    let user_op = input.trim();
    // let obvod = rect.clone().count_perimeter();

    fn process_user_input(input: &str, rect: Rect ) -> Op {
        match input {
            "a" => Op::Area,
            "p" => Op::Perimeter,
            _ => Op::Unknown,
        }
    }

    let processed_input = process_user_input(user_op, user_rect);
    println!("You have entered a: {} b: {}", user_rect.a, user_rect.b );
    println!("You have entered operation: {} ", user_op);
    // println!("Result of operation: {} ", );

    match processed_input {
        Op::Perimeter => println!("Perimeter: {}, ( 2x( {} + {} ))", user_rect.count_perimeter(), user_rect.a, user_rect.b ),
        Op::Area => println!("Area: {}, ({} x {})", user_rect.count_area(), user_rect.a, user_rect.b),
        Op::Unknown => println!("Unknown operation, p - count perimeter , a - count area"),
    }

}