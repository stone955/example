use std::mem;

fn main() {

    // hello world
    {
        // println 不是一个函数，而是一个宏规则
        println!("Hello, world!");
    }

    // format
    {
        let a = 1314;
        println!("a: {}", a);
        println!("a: {0}, a: {0}", a);

        println!("{0}, this is {1}. {1}, this is {0}. ", "Alice", "Bob");

        // As can named arguments
        println!("{subject} {verb} {object}",
                 object = "the lazy dog",
                 subject = "the quick brown fox",
                 verb = "jumps over");

        // Special formatting can be specified after a `:`
        println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

        // You can right-align text with a specified width.
        println!("{number:>width$}", number = 1, width = 4);

        // You can pad numbers with extra zeroes.
        println!("{number:>0width$}", number = 7, width = 3);

        // Rust even checks to make sure the correct number of arguments are used.
        println!("My name is {1}, {0} {1}", "James", "Bond");

        #[derive(Debug)]
        struct Student {
            name: String,
            age: i32,
        }

        let stu = Student {
            name: String::from("Tom"),
            age: 19,
        };

        println!("A student name is {}, age is {}", stu.name, stu.age);
        println!("A student is {:?}", stu);
        println!("A student is {:#?}", stu);
    }


    // scalar types
    {
        // signed integers: i8, i16, i32, i64, i128 and isize(pointer size)
        // unsigned integers: u8, u16, u32, u64, u128 and usize(pointer size)
        // floating point: f32, f64
        // char: Unicode scalar values like 'a', '~' (4 bytes each)
        // bool: either true or false
        // tuple:

        let _logical: bool = true;

        let _float32: f32 = 3.14;
        let _integer64: i64 = 1024;

        let _default_float = 1.732; // `f64`
        let _default_integer = 2048; // `i32`

        let mut _mut_integer = 128;
        _mut_integer = 512;
    }

    // tuples
    {
        // A tuple with a bunch of different types
        let _long_tuple = (
            8u8, 16u16, 32u32, 64u64,
            -8i8, -16i16, -32i32, -64i64,
            0.32f32, 0.64f64,
            true, false,
        );

        // Values can be extracted from the tuple using tuple indexing
        println!("long tuple 1st value: {}", _long_tuple.0);
        println!("long tuple 2nd value: {}", _long_tuple.1);
        println!("long tuple 3rd value: {}", _long_tuple.2);
        println!("long tuple 4th value: {}", _long_tuple.3);
        println!("long tuple 5th value: {}", _long_tuple.4);
        println!("long tuple 6th value: {}", _long_tuple.5);
        println!("long tuple 7th value: {}", _long_tuple.6);
        println!("long tuple 8th value: {}", _long_tuple.7);
        println!("long tuple 9th value: {}", _long_tuple.8);
        println!("long tuple 10th value: {}", _long_tuple.9);
        println!("long tuple 11th value: {}", _long_tuple.10);
        println!("long tuple 12th value: {}", _long_tuple.11);

        // Tuples can be tuple members
        let tuple_of_tuples = (
            (8u8, 16u16, 32u32, 64u64),
            (-8i8, -16i16, -32i32, -64i64),
            (0.32f32, 0.64f64),
            (true, false),
        );

        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // To create one of element tuples, the comma is required to tell them apart
        // from a literal surrounded by parentheses
        let one_tuple = (5u32, );
        println!("one element tuple: {:?}", one_tuple);
        println!("one element tuple: {:?}", (5u32));

        // tuples can be destructured to create bindings
        let tuple = (1, "hello", 4.5, true);
        let (a, b, c, _) = tuple;
        println!("{:?}, {:?}, {:?}", a, b, c);

        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);
    }

    // variables
    {
        let a: u8 = 123; // 8bits
        // a = 456; // error
        println!("a = {}", a);

        // mut
        let mut b: i8 = 1;
        println!("b = {}", b);
        b = 10;
        println!("b = {}", b);

        let c = 123456789;
        println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

        let z: isize = 123; // isize/usize
        let size_of_z = mem::size_of_val(&z);
        println!("z = {}, takes up {} bytes, {}-bits OS",
                 z, size_of_z, size_of_z * 8);

        let d: char = 'x';
        println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

        let e: f64 = 3.14;
        println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

        let t: bool = true;
        println!("t = {}, size = {} bytes", t, mem::size_of_val(&t));
    }

    // operator
    {
        // arithmetic
        let mut a = 2 + 3 * 4;
        println!("a = {}", a);

        a += 1;
        a -= 2;
        println!("{} / {} = {}", a, 3, a / 3);

        let a_cubed = i32::pow(a, 3);
        println!("a_cubed = {}", a_cubed);

        let b = 2.5;
        let b_cubed = f64::powi(b, 3);
        let b_to_pi = f64::powf(b, std::f64::consts::PI);
        println!("b_cubed = {}, b_to_pi = {}", b_cubed, b_to_pi);

        // bitwise
        // | OR & AND ^ XOR ! NOR
        // 01 OR 10 = 11 == 3_10
        let c = 1 | 2;
        println!("1|2 = {}", c);
        // 01 AND 10 = 00 == 0_10
        let d = 1 & 2;
        println!("1&2 = {}", d);
        // 01 XOR 10 = 11 == 3_10
        let e = 1 ^ 2;
        println!("1^2 = {}", e);

        let two_to_10 = 1 << 10;
        println!("2^10 = {}", two_to_10);

        // logical
        let _pi_less_4 = std::f64::consts::PI < 4.0; // true
        let x = 5;
        let _x_is_5 = x == 5; // true
    }

    // scope and shadowing
    {
        let a = 123;
        {
            let a = 777;
            let b = 456;
            println!("inside a = {}", a);
            println!("inside b = {}", b);
        }
        println!("outside a = {}", a);
        // println!("outside b = {}", b); // not found in this scope
    }

    // constants
    {
        const M: u8 = 123;
        println!("const M = {}", M);

        static X: i32 = 456;
        println!("static X = {}", X);

        static mut Z: i32 = 789;
        unsafe {
            Z = 987;
            println!("static Z = {}", Z);
        }
    }

    // stack and heap
    {
        struct Point {
            x: f64,
            y: f64,
        }

        fn origin() -> Point {
            Point {
                x: 0.0,
                y: 0.0,
            }
        }

        let p1 = origin();
        let p2 = Box::new(origin());
        println!("p1 {} bytes", mem::size_of_val(&p1));
        println!("p2 {} bytes", mem::size_of_val(&p2));
        let p3 = *p2;
        println!("p3 {} bytes", mem::size_of_val(&p3));
    }
}
