// Enums are types which have a few definite values

enum Movement {
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(m: Movement) {
  //performa action depending on data
  match m {
    Movement::Up => println!("Avatar moving up"),
    Movement::Down => println!("Avatar moving down"),
    Movement::Left => println!("Avatar moving left"),
    Movement::Right => println!("Avatar moving right"),
  }
}
pub fn run() {
  //
  let avatar_u = Movement::Up;
  let avatar_d = Movement::Down;
  let avatar_l = Movement::Left;
  let avatar_r = Movement::Right;

  move_avatar(avatar_u);
  move_avatar(avatar_d);
  move_avatar(avatar_l);
  move_avatar(avatar_r);

  println!("");
}