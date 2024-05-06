/// pin and unpin in rust

pub fn normal_moves() {
    let a = String::from("Hello");
    // show memory address of a
    println!("a memory address: {:p}", a.as_ptr());
    let b = a;
    println!("b memory address: {:p}", b.as_ptr());
    
    let num = 5;
    let num_ref = &num;
    println!("num: {:p}", num_ref);
    println!("num_ref: {:p}", &num_ref);

    let mut x = String::from("XXX");
    let mut y = String::from("YYY");
    println!("x: {:p} y: {:p}", x.as_ptr(), y.as_ptr());

    std::mem::swap(&mut x, &mut y);
    println!("x: {:p} y: {:p}", x.as_ptr(), y.as_ptr());
    println!("x: {}, y: {}", x, y);
}

pub fn try_self_referential_struct() {
    struct Test<'a> {
        a: String,
        b: &'a String,
    }
    let a = String::from("Hello");
    // let test = Test {
    //     a,
    //     b: &a,
    // };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        normal_moves();
    }
}