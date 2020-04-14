#![allow(dead_code)]
#![allow(unused_variables)]

pub fn if_statement() {
    let temp = 40;

    if temp > 20 {
        println!("Really hot");
    } else if temp < 10 {
        println!("Really cold");
    } else {
        println!("OK");
    }

    let day = if temp > 20 { "sunny" } else if temp < 10 { "cloudy" } else { "OK" };
    println!("DAY is {}", day);
}

pub fn loops() {
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64 { continue; }
        println!("X = {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

pub fn for_loops() {
    for x in 1..11 {
        if x == 3 { continue; }
        if x == 8 { break; }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub fn matches() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        45 => "US",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("country is {}, and it code is {}", country_code, country);
}

pub fn option() {
    let x = 2.0;
    let y = 0.0;

    // Option -> Some(x/y) | None
    let result =
        if y != 0.0 { Some(x/y) } else { None };
    
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide by Zero")
    }

    // IF result match with Some(z), this will executed.
    if let Some(z) = result {
        println!("Result = {}", z)
    }

    while let Some(z) = result {
        println!("Result = {}", z)
    }
}
