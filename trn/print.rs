pub fn run() {
  // Run function for print.rs
  // uses standard placeholder with string and expression,
  // named placeholder, trait placeholder, positional placeholder
  // result is "Helo from print.rs, test 1 - red, 22 - 1"
  println!("Helo from {}, test {} - {ginger}, {hex:x} - {1}", "print.rs", 0+1, hex = 34, ginger = "red");
}