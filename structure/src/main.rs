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
    Cmy { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}

fn main() {
    // struct
    {
        let o = Point {
            _x: 0.0,
            _y: 0.0,
        };
        println!("Point o is at ({}, {})", o._x, o._y);

        let line = Line {
            _start: Point {
                _x: 0.0,
                _y: 0.0,
            },
            _end: Point {
                _x: 3.0,
                _y: 4.0,
            },
        };
        println!("Line start {:?}, end {:?}", line._start, line._end);
    }

    // enumeration
    {
        let light = Light::Red; // Color::Yellow Color::Green
        let active = match light {
            Light::Red => "Stop",
            Light::Yellow => "Ready",
            Light::Green => "Go"
        };
        println!("The light is {:?}, {}!", light, active);

        // let rgb = Color::Rgb(0, 0, 0);
        // let rgb = Color::Cmy { cyan: 0, magenta: 0, yellow: 0, black: 255 };
        let rgb = Color::Cmy { cyan: 0, magenta: 0, yellow: 0, black: 0 };
        match rgb {
            Color::Rgb(0, 0, 0)
            | Color::Cmy { cyan: _, magenta: _, yellow: _, black: 255 } => {
                println!("Color is Black")
            }
            Color::Rgb(r, g, b) => println!("Color rgb is {}, {}, {}", r, g, b),
            _ => println!("Color is what?")
        };
    }
}
