use std::{
    fs::File,
    io::{Read, Write},
};

use tokio::sync::{Semaphore, SemaphorePermit};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    solgan: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Unknown Name".to_string(),
            age: 0,
            solgan: "Talk is cheep, show me the code".to_string(),
        }
    }
}

impl User {
    pub fn new(name: String, age: u32, solgan: String) -> Self {
        User {
            name: name,
            age: age,
            solgan: solgan,
        }
    }

    pub fn persist(&self, filename: String) -> Result<usize, std::io::Error> {
        let mut file = File::create(filename)?;

        // serialize User struct
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        print!("{}", data);

        Ok(data.len())
    }

    pub fn load(filename: String) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        // Deserialize
        let user = serde_json::from_str(&data)?;

        Ok(user)
    }
}

// lifetime
pub fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
    match s.find(delimiter) {
        Some(i) => {
            let prefix = &s[..i];
            let suffix = &s[i + delimiter.len_utf8()..];
            *s = suffix;
            prefix
        }
        None => {
            let prefix = *s;
            *s = "";
            prefix
        }
    }
}

/// **Lifetime Example**
/// museum tickets 
pub struct Museum {
    remaining_ticket: Semaphore,
}

impl Museum {
    fn new(max_ticket: usize) -> Self {
        Museum {
            remaining_ticket: Semaphore::new(max_ticket),
        }
    }

    fn get_ticket(&self) -> Option<Ticket<'_>> {
        match self.remaining_ticket.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    fn avaialable_ticket(&self) -> usize {
       self.remaining_ticket.available_permits()
    }
}

pub struct Ticket<'a> {
    ticket: SemaphorePermit<'a>,
}

impl Drop for Ticket<'_> {
   fn drop(&mut self) {
       println!("Ticket dropped");
   }
}

impl<'a> Ticket<'a> {
    pub fn new(ticket: SemaphorePermit<'a>) -> Self {
        Self { ticket }
    }
}

/// **Generic Trait Example**
/// Event Encoder
pub trait EventEncoder {
    fn encode(&self) -> String;
}

pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data> where Id: EventEncoder, Data: EventEncoder {
    pub fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }
    pub fn encode(&self) -> String {
        format!("{} {}", self.id.encode(), self.data.encode())
    }
}

impl EventEncoder for String {
    fn encode(&self) -> String {
        self.to_string()
    }
}

impl EventEncoder for u32 {
    fn encode(&self) -> String {
        self.to_string()
    }
}


#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::train_code::strtok;

    #[test]
    fn user_work() {
        let filepath = "data/persist-user.json";
        let user = User::default();
        user.persist(filepath.to_string()).unwrap();

        let user2 = User::load(filepath.to_string()).unwrap();
        assert!(user == user2);
    }

    #[test]
    fn use_strtok() {
        let mut s = "hello world";
        assert_eq!(s.find(" ").unwrap(), 5);

        let prefix = strtok(&mut s, ' ');
        assert_eq!(prefix, "hello");
        assert_eq!(s, "world");

        // let a = "hello world".to_owned();
        // thread::spawn(move || {
        //     println!("{}", a);
        // });
    }


    #[test]
    fn museum_work() {
       let museum = Museum::new(10);
       let ticket1 = museum.get_ticket().unwrap();
       assert_eq!(museum.avaialable_ticket(), 9);

       drop(ticket1);

       let ticket_lists: Vec<Ticket> = (0..10).map(|i| museum.get_ticket().unwrap()).collect();

       assert!(museum.get_ticket().is_none());
       print!("--------------------------------")
    }


    #[test]
    fn event_work() {
        let event = Event::new(10u32, "hello world".to_owned());
        assert_eq!(event.encode(), "10 hello world");
    }
}
