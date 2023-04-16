//arrays are fixed list where elements are the same data type
use std::mem;
pub fn run() {

  let mut numbers: [i32; 5] = [1,2,3,4,5];

  numbers[2]=7;


  println!("{:?}", numbers );
  println!("single value: {}",numbers[0]);
  //Lenth - numbers.len()
  //stack allocated (bytes) - 
  println!("{}", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

}