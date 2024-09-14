fn main() {
    // variables by default are immutable and mut keyword needs to be added to make it mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    //constants are created using const keyword
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // local variable can be created with the same name as global but limited to that scope
    // shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // ---> 12
    }
    println!("The value of x in the inner scope is: {x}"); // ---> 6

    // tuple
    let tup = (2, 5.2, false);
    let (x, y, z) = tup;  // destructuring

    println!("The value of x, y, z are: {x} {y} {z}");

    let x = tup.0;  // accessing element using index 
    let y = tup.1;
    let z = tup.2;

    println!("The value of x, y, z are: {x} {y} {z}");

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // declaring dataype and size
    let first = a[0]; // accessing element using index
    let second = a[1];

    let a = [3; 5]; // creates array of 5 elements with value 3 ---> [3, 3, 3, 3, 3];

    another_function(3);

    let x = five();
    println!("Value of x is: {x}");

    // loops
    loop_keyword();
    while_loop();
    for_loop();

    //references
    let mut s = String::from("hello");

    let r1 = &s; // creating immutable reference
    let r2 = &s; // no problem in creating another immutable reference
    //let r3 = &mut s; // BIG PROBLEM as we dont want dirty read in immutable refs

    println!("{}, {}", r1, r2);
    
}


// function follow snake casing
fn another_function(x: i32) {
    println!("Value of x is: {x}");
}

// function with return type
fn five() -> i32 {
    5  // return value --> should not end with semi colon
}

fn loop_keyword() {
    let mut counter = 0;
    let result = loop {   //loop keyword to create infinte loop until a break statement is encountered
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {result}");
}

fn while_loop() {
    let mut number = 3;
    while number > 0 {
        number -= 1;
    }
    println!("Lift off!");
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5 ];
    for element in a {
        print!("{element} ");
    }
    println!();

    for number in (1..4).rev() {
        print!("{number}! ");
    }
    println!("LIFTOFF!!!");
}
