use std::mem;

fn main() {

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
        println!("a pow 3 = {}", a_cubed);
    }
}
