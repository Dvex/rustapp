#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::mem;

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tupla
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8} // struct
}

union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => { println!("meaning of live") },
            IntOrFloat { f } => { println!("f value is {}", f) }
        }
    }
}

fn enums() {
    // let c::Color = Color::Blue;
    // let c::Color = Color::RgbColor(0,0,0);
    let c:Color = Color::Cmyk {cyan: 0, magenta: 12, yellow: 10, black: 0};

    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RgbColor(0,0,0)
        | Color::Cmyk {cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb {},{},{}", r, g, b),
        _ => ()
    }
}

pub fn make_union () {
    let mut iof = IntOrFloat{ i: 123 };
    iof.i = 234;

    let variable = unsafe { iof.i };
    println!("iof.i value {}", variable);

    process_value(IntOrFloat{i:42});
}