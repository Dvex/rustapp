#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;
mod statements;
mod data;
use std::mem;
use use_crate;

const MEAMING_OF_LIFE:u8 = 42;
static MY_AGE:i32 = 26;
static mut MY_FUTURE_AGE:i32 = 27;

fn main() {
    // scope_and_shadowing();
    // println!("MY_MEAMING_OF_LIFE {}", MEAMING_OF_LIFE);
    // println!("MY_AGE is {}", MY_AGE);
    // unsafe
    // {
        // MY_FUTURE_AGE = 78;
        // println!("MY_FUTURE_AGE will {}", MY_FUTURE_AGE);
    // }
    // FROM OTHER MODULES
    // sh::stack_and_heap();
    // statements::if_statement();
    // statements::loops();
    // statements::for_loops();
    // statements::matches();
    // data::make_union();
    // statements::option();
}

fn data_type () {
    // DATA TYPES
    let a:u8 = 123;
    let b:i8 = -123;
    let mut c:u8 = 1;
    println!("Value of inmmutable variable a is {}. Val of muttable variable c is {}", a, c);
    println!("Value of inmmutable variable b is {}", b);
    c = 200;
    println!("Val of muttable variable c is {}", c);
    let mut d = 123345596;
    println!("Val of muttable variable d is {} and size in bytes is {}", d, mem::size_of_val(&d));
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    // CHARS
    let e = 'x';
    println!("Val of muttable variable e is {} and size in bytes is {}", e, mem::size_of_val(&e));
    // FLOAT / DOUBLE
    let f = 2.5; // 64bits (double-precision / f64) or 32bits (single-precision / f32)
    println!("Val of muttable variable f is {} and size in bytes is {}", f, mem::size_of_val(&f));
    // BOOLEANS
    let g = false;
    let h = 1>0;
}

fn operatos () {
    // OPERATOR
    let x = 2.5; // + - * / += -= *= /= %=
    let z = 2+3+4-1;
    let z_cubed = i32::pow(z, 2);
    let y_cubed = f64::powi(x, 2); // to integer number
    let o_cubed = f64::powf(x, std::f64::consts::PI); // to float/double number
    println!("{} x_cubed, {} y_cubed, {} o_cubed", z_cubed, y_cubed, o_cubed);

    // bitwise
    let g = 1 | 2; // | OR, & AND, ^ XOR, ! NOR
    let v = 1 << 10; // >>
    println!("{} is 1^10", v);

    // logical
    let pi_lees_4 = std::f64::consts::PI < 4.0; // true
    // > < == >= <=
}

fn scope_and_shadowing () {
    let a = 123;
    // let a = 444;
    {
        let b = 456;
        println!("b value inside {}", b);

        let a = 999;
        println!("a value inside {}", a);
    }
    println!("a value outside {}", a);
}