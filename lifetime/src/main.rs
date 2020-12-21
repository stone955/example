fn main() {
    // ownership
    {
        /*
        let v = vec![1, 2, 3];
        let v2 = v; // value moved here
        println!("v = {:?}", v); // value borrowed here after move
         */

        /*
        let u = 1;
        let u2 = u;
        println!("u = {}", u);
        */

        /*
        let u = Box::new(1);
        let u2 = u; // value moved here
        println!("u = {}", u); // value borrowed here after move
         */
    }

    // borrowing
    {
        let print_vector = |x: &Vec<i32>| {
            println!("x[0] = {}", x[0]);
        };

        let v = vec![1, 2, 3];
        print_vector(&v);
        println!("v[0] = {}", v[0]);

        let mut a = 50;
        {
            let b = &mut a;
            *b += 10;
        }
        println!("a = {}", a);
    }
}
