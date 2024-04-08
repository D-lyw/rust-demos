use std::{fs::File, io::{Read, Write}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
  name: String,
  age: u32,
  solgan: String
}

impl Default for User {
    fn default() -> Self {
        Self { name: "Unknown Name".to_string(), age: 0, solgan: "Talk is cheep, show me the code".to_string() }
    }
}

impl User {
    pub fn new(name: String, age: u32, solgan: String) -> Self {
        User { name: name, age: age, solgan: solgan }
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

#[cfg(test)]
mod tests {
  use super::User;

  #[test]
  fn user_work() {
      let filepath = "data/persist-user.json";
      let user = User::default();
      user.persist(filepath.to_string()).unwrap();

      let user2 = User::load(filepath.to_string()).unwrap();
      assert!(user == user2);
  }
}