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

fn main() {
    // Point
    {
        let o = Point {
            _x: 0.0,
            _y: 0.0,
        };
        println!("Point o is at ({}, {})", o._x, o._y);
    }

    // Line
    {
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
}
