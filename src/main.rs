mod r#struct;
mod basic;


use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Range};
use std::time::{Duration, Instant};
use num::Complex;
use rand::Rng;
use server::show_name;
use crate::basic::{arrays_slices, enum_usage, init_val, macro_usage, ownership_move, print_min_width, string_num_convert, val_shadowing, vector_usage};

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    age: u8,
    vip: bool
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    // guess_number();
    // let user1 = User {
    //     email: String::from("xyz@mail.com"),
    //     username: String::from("xyz"),
    //     age: 18,
    //     vip: true,
    // };
    // println!("{:?}", user1);
    //
    // let rect1 = Rectangle{
    //     width: 10,
    //     height: 6,
    // };
    // println!("area is {}", rect1.area());
    // init_val();
    //
    //
    // val_shadowing();
    //
    // assert_eq!(i8::MAX, 127);
    // show_range();
    //
    // let xx = 12;
    // println!("{:p}", &xx);

    // count_time();

    // println!("Check number type: {}", check_is_even(13));

    // let mandelbrot = calculate_mandelbrot(1000, 2.0, 1.0, -1.0, 1.0, 100, 24);
    // render_mandelbrot(mandelbrot);
    //
    // let floats = add(1.3, 2.9);
    // println!("{}", floats);
    //
    // read_file();
    // println!("{}", print_min_width());

    arrays_slices();

    vector_usage();
    macro_usage();
    // enum_usage();
    string_num_convert();

    // show_name("Server Lib show name function".to_string());
    ownership_move();
}

fn read_file() {
    let f = File::open("src/basic.rs").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        // println!("{} ({} bytes long)", line, line.len());
        if (line.contains("println")) {
            println!("{}", line);
        }
    }
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn reporter<T: Debug>(item: T) {
    println!("{:?}",item);
}


fn count_time() {
    let mut count = 0;
    let time_range = Duration::new(1,0);
    let start_time = Instant::now();

    while (Instant::now() - start_time) < time_range {
        count += 1;
    }
    println!("Count Number: {}", count);
}

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);

    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(
    cx: f64,
    cy: f64,
    max_iters: usize
) -> usize {
    let mut z = Complex {re: 0.0, im: 0.0};
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '~',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn check_is_even(num: i32) -> String {
    match is_even(num) {
        true => String::from("even"),
        false => String::from("odd"),
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}


fn show_range() {
    let mut sum = 0;
    for i in -3..3 {
        sum += i;
        println!("{}", i);
    }
    assert_eq!(sum, -3);

    assert_eq!((1..5), Range{ start:1, end: 5});
}










fn guess_number() {
    println!("Guess Number Game!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

        println!("Please input your guess number:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail to read line");

        // string format to number type
        let guess_number: u32 = guess.trim().parse().expect("Please input number");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You are right!");
                break;
            }
        }
    }
}

