//vectors are resizeable arrays
use std::mem;
pub fn run() {

  let mut numbers: Vec<i32> = vec![1,2,3,4,5];

  numbers[2]=7;

  //add on to vector
  numbers.push(6);

  //pop off last value
  numbers.pop();

  println!("{:?}", numbers );
  println!("single value: {}",numbers[0]);
  //Lenth - numbers.len()
  //stack allocated (bytes) - 
  println!("{}", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // loop through values
  for x in numbers.iter() {
    println!{"number: {}", x}
  }
  for x in numbers.iter_mut() {
    // multiply each by two
    *x *= 2;
    println!{"number: {}", x}
  }
 
  println!{"Numbers Vec: {:?}", numbers}
}