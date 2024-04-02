use std::ops::Add;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn show_name(name: String) -> String {
    let mut prefix = String::from("My name is: ");
    prefix.add(&name)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_name() {
        let name = String::from("xyz");
        assert_eq!(show_name(name), "My name is: xyz");
    }
}
