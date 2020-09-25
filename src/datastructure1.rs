pub fn data_structures_1() {
    struct_play();
    enum_play();
    option_play();
}

struct Employee {
    name: &'static str,
    age: i16,
}

struct Department {
    manager: Employee,
    developer: Employee,
}

enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8),
    //tuple
    CmykColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },//struct
}

fn struct_play() {
    let manager = Employee { name: "Vince McMahon", age: 78 };
    let developer = Employee { name: "The Rock", age: 48 };
    let wwe = Department { manager, developer };
    println!("wwe's manager is {} and developer is {}", wwe.manager.name, wwe.developer.name);
}

fn enum_play() {
    let color: Color = Color::CmykColor { cyan: 1, magenta: 2, yellow: 3, black: 4 };

    match color {
        Color::Red => println!("Reg"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RgbColor(_, _, _) => println!("RgbColor"),
        Color::CmykColor { cyan: c, magenta: m, yellow: y, black: b } => println!("CmykColor {}", c)
    }
}

fn option_play() {
    let x = 4;
    let y = 0;
    let result: Option<i32> = if y != 0 { Some(x / y) } else { None };

    println!("Option Result : {:?}", result);

    match result {
        Some(z) => println!("Result : {}", z),
        None => {
            println!("not possible");
            println!("Yes correct. Read previous line.");
        }
    }

    if let Some(z) = result {
        println!("z {}", z);
    } else {
        println!("None");
    }
}