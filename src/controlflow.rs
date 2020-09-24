use std::iter::Enumerate;

pub fn if_statement() {
    let age = 19;
    if age > 18 {
        println!("Adult");
    } else {
        println!("Minor");
    }

    let adultOrMinor = if age > 18 {
        "Adult"
    } else {
        "Minor"
    };
    println!("{}", adultOrMinor);
}

pub fn while_and_loop() {
    let mut x = 2;
    loop {
        x = x * 2;
        println!("x = {}", x);
        if x > 10000 {
            break;
        }
    }
}

pub fn for_loop() {
    for x in 1..20 {
        println!("{}", x);
    }

    for (pos, y) in (50..61).enumerate() {
        println!("{} : {}", pos, y);
    }
}

pub fn match_statement() {
    let num = 9;
    let numStr = match num {
        1 => "One",
        2 => "Two",
        3...10 => "Three to Ten",
        _ => "Default"
    };
    println!("numStr = {}", numStr);
}