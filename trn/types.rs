/*
Primitive types--
Integers: u8, i8, u16, i16, ... u128, i128

Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays

Statically typed, but compiler infers

 */

pub fn run() {

// default i32
let x = 1;

// default f64
let y = 2.5;

// explicit
let z: i64 = 454545454544;

//find max size
println!("Max i32: {}", std::i32::MAX);
println!("max i64: {}", std::i64::MAX);

//Boolean
let is_active = true;

//get boolean from expression
let is_greater: bool = 10 < 5;

let a1 = 'a';

let face = '\u{1f600}';

println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}