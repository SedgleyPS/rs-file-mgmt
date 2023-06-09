//functions - stores blocks of code for re-use

pub fn run() {
  //
  greeting("Hello","Jane");

  // bind functions to variables
  let get_sum = add(5,5);
  println!("Sum: {}", get_sum);

  //closure
  let n3: i32 = 10; // outside variable
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; //can use outside variables
  println!("C Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name);
}

// -> - this sets a return type
fn add(n1: i32, n2: i32,) -> i32 {
  n1 + n2  // not using a semicolon allows it to return the value
}