fn main() {
    // println 不是一个函数，而是一个宏规则
    println!("Hello, world!");

    // 格式化输出
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
    println!("{number:>0width$}", number = 7, width= 3);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {1}, {0} {1}", "James", "Bond");

    #[derive(Debug)]
    struct Student {
        name : String,
        age : i32
    }

    let stu = Student{
        name : String::from("Tom"),
        age : 19
    };

    println!("A student name is {}, age is {}", stu.name, stu.age);
    println!("A student is {:?}", stu);
    println!("A student is {:#?}", stu);
}
