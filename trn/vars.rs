// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block scoped language


pub fn run() {
  let first_name = "Cool";
  let mut first_initial = "E";
  println!("Ladies love {} {}", first_name, first_initial); 
  first_initial = "J";
  println!("Ladies love {} {}", first_name, first_initial);
}