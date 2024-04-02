use std::fmt::Display;
use std::ops;

pub fn init_val() {
    let x: u32 = 10;
    let y: u32 = 20;
    println!("{}, {}", x, y);
}

pub fn val_range() {
    let x = 10;
    {
        let y = 20;
        println!("x: {}, y: {}", x, y);
    }
    println!("x: {}, y: {}", x, 2);
}

pub fn val_shadowing() {
    let x = 10;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 10);
    let x = 20;
    assert_eq!(x, 20);
    println!("x: {}", x);
}

pub fn print_min_width() -> String {
    // 设置变量输出最小宽度
    format!("{:>5}", 10)
}
pub fn print_min_width_with_left_fill() -> String {
    format!("{:0>5}", 10)
}
pub fn print_min_width_with_right_fill() -> String {
    format!("{:0<5}", 10)
}
pub fn print_number_with_binary(num: i32) -> String {
    format!("{:b}", num)
}
pub fn print_number_with_octal(num: i32) -> String {
    format!("{:o}", num)
}
pub fn print_number_with_hex(num: i32) -> String {
    format!("{:x}", num)
}

pub fn arrays_slices() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];

    assert_eq!(slice[0], 2);
    assert_eq!(slice[1], 3);
    assert_eq!(slice[2], 4);
    assert_eq!(slice.len(), 3);
}

pub fn vector_usage() {
    let mut vec:Vec<i32> = Vec::new();
    for val in 0..10  {
        vec.push(val);
    }
    assert_eq!(vec[1], 1);
    println!("{:?}", vec);

    if let Some(last) = vec.pop() {
        println!("Last item: {}", last);
    }

    match vec.get(3) {
        Some(val) => println!("Value index of 3: {}", val),
        None => println!("None value")
    }

    for value in &vec {
        println!("{}", value);
    }

    println!("Capacity: {}", vec.capacity());
}

pub fn macro_usage() {
    macro_rules! create_function {
        ($fun_name:ident, $message: expr) => {
            fn $fun_name() {
                println!("{}", $message);
            }
        };
    }
    create_function!(hallo, "hello, macro function");
    hallo();
}

pub fn enum_usage() {
    enum List {
        Cons(u32, Box<List>),
        Nil
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, elem: u32) -> List {
            List::Cons(elem, Box::new(self))
        }

        fn lens(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.lens(),
                List::Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{} {}", head, tail.stringify())
                },
                List::Nil => {
                    "Nil".to_string()
                }
            }
        }
    }

    let mut list = List::new();
    list = list.prepend(12);
    list = list.prepend(23);
    list = list.prepend(34);
    list = list.prepend(45);

    println!("List length: {}", list.lens());
    println!("{}", list.stringify());
}

pub fn string_num_convert() {
    struct Circle {
        radius: i32
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", format!("Circle of Radius {}", self.radius))
        }
    }

    let c = Circle { radius: 10 };
    println!("{:?}", c.to_string());
}

pub fn flow_of_control() -> i32 {
    let mut count = 1;
    loop {
        count += 1;
        if (count == 3) {
            println!("Three");
            continue;
        }
        if (count == 10) {
            println!("Enough");
            break;
        }
    }
    println!("Finish Loop");
    count
}

pub fn ownership_move() {
    let x: u32 = 7;
    let y = x;
    println!("{:p}, {:p}", &x, &y);
}

pub fn borrow_mutability() {
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32
    }

    // read only borrow
    fn borrow_book(book: &Book) {
        println!("Now I'm reading {}", book.title);
    }

    // read and write borrow
    fn edit_book(book: &mut Book) {
        book.year = book.year + 1;
        println!("New year of book: {}", book.year);
    }

    let immutbook = Book {
        author: "D-lyw",
        title: "The Book Of Questions",
        year: 2023
    };

    let mut mutablebook = immutbook;

    // borrow_book(&immutbook);
    // borrow_book(&mutablebook);
    // edit_book(&immutbook);
    edit_book(&mut mutablebook);
}

fn operator_overload() {

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct  FooBar;

    #[derive(Debug)]
    struct BarFoo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo");
        }
    }

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, rhs: Bar) -> Self::Output {
            FooBar
        }
    }

    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, rhs: Foo) -> Self::Output {
            BarFoo
        }
    }

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

fn traits_iter() {
    #[derive(Copy, Clone)]
    struct Fibonacci {
        cur: u32,
        next: u32
    }

    impl Fibonacci {
        fn new() -> Fibonacci {
            Fibonacci { cur: 0, next: 1}
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.cur;
            self.cur = self.next;
            self.next = current + self.next;

            if (current > 100) {
                None
            } else {
                Some(current)
            }
        }
    }

    let fibonacci = Fibonacci::new();

    for i in fibonacci.into_iter() {
        println!("> {}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_usage() {
        assert_eq!(print_min_width(), "   10");
        assert_eq!(print_min_width_with_left_fill(), "00010");
        assert_eq!(print_min_width_with_right_fill(), "10000");
        assert_eq!(print_number_with_binary(69420), "10000111100101100");
        assert_eq!(print_number_with_octal(69420), "207454");
        assert_eq!(print_number_with_hex(69420), "10f2c");
    }

    #[test]
    fn test_array_slice() {
        arrays_slices();
    }

    #[test]
    fn test_vector_usage() {
        vector_usage();
    }

    #[test]
    fn test_flow_of_control() {
        assert_eq!(flow_of_control(), 10);
    }

    #[test]
    fn test_operator_overload() {
        operator_overload();
    }


    #[test]
    fn test_traits_iter() {
        traits_iter();
    }
}