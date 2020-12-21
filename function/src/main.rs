fn main() {
    // function
    {
        // call function
        fn print_value(x: i32) {
            println!("x = {}", x);
        }
        print_value(33);
    }

    // closure
    {
        let pow = |x: i32| -> i32 {
            x * x
        };

        let x = 3;
        println!("x pow is {}", pow(x));
    }

    // method
    {
        struct Point {
            x: f64,
            y: f64,
        }

        struct Line {
            start_point: Point,
            end_point: Point,
        }

        impl Line {
            fn length_of_line(&self) -> f64 {
                let x = self.end_point.x - self.start_point.x;
                let y = self.end_point.y - self.start_point.y;
                let z = x * x + y * y;
                z.sqrt()
            }
        }

        let line = Line {
            start_point: Point { x: 0.0, y: 0.0 },
            end_point: Point { x: 3.0, y: 4.0 },
        };

        println!("length of line is {}", line.length_of_line());
    }

    // trait
    {
        trait Animal {
            fn name(&self) -> &'static str;
            fn fly(&self) {
                println!("{} can not fly", self.name());
            }
            fn run(&self) {
                println!("{} can not run", self.name());
            }
        }

        struct Bird {
            name: &'static str
        }

        impl Animal for Bird {
            fn name(&self) -> &'static str {
                self.name
            }
            fn fly(&self) {
                println!("{} is flying", self.name());
            }
            fn run(&self) {
                println!("{} can not run", self.name());
            }
        }

        struct Human {
            name: &'static str
        }

        impl Animal for Human {
            fn name(&self) -> &'static str {
                self.name
            }
            fn fly(&self) {
                println!("{} can not fly", self.name());
            }
            fn run(&self) {
                println!("{} is running", self.name());
            }
        }

        let bird = Bird {
            name: "Anderson"
        };
        bird.fly();
        bird.run();

        let human = Human {
            name: "John"
        };
        human.fly();
        human.run();
    }
}
