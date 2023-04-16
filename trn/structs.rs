// Structs - used to create custom data types

//tradtional struct
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

//tuple struct
struct Color2 (u8,u8,u8);

//struct with functions

struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  // Construct person
  fn new(first: &str,last: &str) -> Person {
    Person {
      first_name: first.to_string(), // convert type from String to str
      last_name: last.to_string() 
    }
  }

  //get Full Name
  fn full_name(&self) -> String {
    format!("{}, {}",self.last_name,self.first_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // use tuple
  // Name to Tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }

}

pub fn run() {
  //
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0
  };

  c.red = 200;
  println!("Color: {} {} {}",c.red,c.green,c.blue);

  let mut c2 = Color2(255,0,0);
  c2.2 = 200;

  println!("Color2: {} {} {}",c2.0,c2.1,c2.2);

  let mut p = Person::new("Mary","Doe");

  println!("Person: {} {}", p.first_name, p.last_name);
  println!("Person: {}", p.full_name());

  p.set_last_name("Williams");
  println!("Person: {}", p.full_name());

  println!("PersonT: {:?}", p.to_tuple());
  
}