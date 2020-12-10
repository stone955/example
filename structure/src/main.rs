use std::mem;

#[derive(Debug)]
struct Point {
    _x: f64,
    _y: f64,
}

#[derive(Debug)]
struct Line {
    _start: Point,
    _end: Point,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Light {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    // tuple
    Rgb(u8, u8, u8),
    // struct
    Cmy {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn main() {
    // struct
    {
        let o = Point { _x: 0.0, _y: 0.0 };
        println!("Point o is at ({}, {})", o._x, o._y);

        let line = Line {
            _start: Point { _x: 0.0, _y: 0.0 },
            _end: Point { _x: 3.0, _y: 4.0 },
        };
        println!("Line start {:?}, end {:?}", line._start, line._end);
    }

    // enumeration
    {
        let light = Light::Red; // Color::Yellow Color::Green
        let active = match light {
            Light::Red => "Stop",
            Light::Yellow => "Ready",
            Light::Green => "Go",
        };
        println!("The light is {:?}, {}!", light, active);

        // let rgb = Color::Rgb(0, 0, 0);
        // let rgb = Color::Cmy { cyan: 0, magenta: 0, yellow: 0, black: 255 };
        let rgb = Color::Cmy {
            cyan: 0,
            magenta: 0,
            yellow: 0,
            black: 0,
        };
        match rgb {
            Color::Rgb(0, 0, 0)
            | Color::Cmy {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => {
                println!("Color is Black")
            }
            Color::Rgb(r, g, b) => println!("Color rgb is {}, {}, {}", r, g, b),
            _ => println!("Color is what?"),
        };
    }

    // option
    {
        // Option<T>
        let x = 3.0;
        let y = 2.0;

        // Some(z) None
        let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };
        match result {
            Some(z) => {
                println!("{}/{} = {}", x, y, z);
            }
            None => {
                println!("{}/{} = #REF", x, y);
            }
        }
    }

    // array
    {
        let mut a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("a has {} elements, first is {}", a.len(), a[0]);

        // mut
        a[0] = 6;
        println!("a[0] = {}", a[0]);

        // for... in
        for i in 0..a.len() {
            println!("a[{}] = {}", i, a[i]);
        }

        let b = [1; 10];
        for i in 0..b.len() {
            println!("b[{}] = {}", i, b[i]);
        }
        println!("b took up {} bytes", mem::size_of_val(&b)); // 40 bytes

        let mtx: [[i32; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        for i in 0..mtx.len() {
            for j in 0..mtx[i].len() {
                print!("{} ", mtx[i][j]);
            }
            println!();
        }
    }

    // vector
    {
        let mut v = Vec::new();
        // push
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
        println!("v = {:?}", v);

        println!("v[0] = {}", v[0]);

        // not safe
        // println!("v[6] = {}", v[6]); // panicked at 'index out of bounds: the len is 5 but the index is 6',

        // safe using option
        let idx = 4;
        match v.get(idx) {
            Some(x) => println!("v[{}] = {}", idx, x),
            None => println!("error, no such element of index {}", idx),
        }

        // for... in...
        for x in &v {
            println!("{}", x);
        }

        // pop
        match v.pop() {
            Some(x) => println!("the last element is {}, v = {:?}", x, v),
            None => println!("vector is empty"),
        }

        while let Some(x) = v.pop() {
            println!("{}", x);
        }
    }
}
