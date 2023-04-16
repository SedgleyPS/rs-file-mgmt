// primitive str - immutable fixed length
// string = growable, heap-allocated data structure - use when you need to modify or own data


pub fn run() {
  let mut hello = String::from("Hello ");

  println!("Length: {}", hello.len());

  hello.push('W'); //single char
  hello.push_str("orld!"); //add string
  
  println!("capacity: {}", hello.capacity());
  println!("is empty: {}", hello.is_empty());
  println!("contains 'World' {}", hello.contains("World")); //contains is case-sensitive
  println!("Replace: {}", hello.replace("World", "There"));
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing - compiler will stop if value isn't as expected.
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);

  println!("{}",hello);

}