fn how_many(x: u32) -> &'static str {
    match x {
        0 => "no",
        1..=10 => "a few",
        _ => "lots of",
    }
}

pub fn pattern_matching() {
    for x in 0..20 {
        println!("{}, I have {} apples!", x, how_many(x));
    }

    let point = (0, 4);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("x = {}, y axis", x),
        (x, y) => println!("x = {}, y = {}", x, y),
    }
}
