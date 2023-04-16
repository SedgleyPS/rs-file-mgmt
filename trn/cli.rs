use std::env;

pub fn run() {
  // 
  let args: Vec<String> = env::args().collect();
  
   
  if args.len() > 1 {
    let command = args[1].clone();
    let status = "100%";
    let mut name = String::from("Unknown");  //default
    println!("Command: {}",command);
    if args.len() > 2 {
      name = args[2].clone();     
    }
    if command == "hello" {
      println!("Hi {}, Good day!",name);
    }
    else if command == "status" {
      println!("Status is {}",status);
    } else {
      println!("Invalid Command");
    }
  }
  else {
    println!("Args: {:?}",args);
  }
}