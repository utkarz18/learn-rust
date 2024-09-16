enum Coin {
    Penny,
    Dime,
    Quater(i32),
}

impl Coin {
    fn display_msg(&self) {
        println!("you have a currency!");
    }
}

fn main() {
    let x = Some(5);
    //let x: Option<i32> = None;

    match x {
        Some(i) => println!("{i}"),
        None => println!("No value")
    };


    let p = Coin::Penny;
    let d = Coin::Dime;
    let q = Coin::Quater(25);

    q.display_msg();
    println!("{}", match_expr(&p));
    println!("{}", if_let(&d));

}

fn match_expr(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Quater(i32) => 25,
    }
}

fn if_let(coin: &Coin) -> i32 {
    if let Coin::Dime = coin {
        println!("Yo got a Dime!");
        10
    } else {
        println!("Yo have something else!");
        0
    }
}


