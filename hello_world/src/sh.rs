#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

// ONLY accept one type of data
struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap () {
    // IN STACK
    let p1 = origin();
    // IN HEAD
    let p2 = Box::new(origin());
    
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // BACK TO THE STACK
    let p3 = *p2;
    println!("p3.x is {}", p3.x);
}

// GENERICS -> THIS can accept many type of data

struct PointT<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: PointT<T>,
    end: PointT<T>
}

fn generics() {
    // let a = PointT { x:0, y:0 };
    let a = PointT { x:0.0, y: 4f64 };
    let b = PointT { x:1.2, y: 3.4  };

    let line = Line { start: a, end: b };
}

// SLICES

fn slices() {
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);
}

fn use_slice(slice: &mut[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 1234;
}

// MATCHS PATTERNS

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} things", x, how_many(x));
    }
}

fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        // z @ 9..1 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}