// 4.6
// fn main_old() {
//   let s = String::from("hello");

//   change(s);
// }

// fn change_old(some_string: String) {
//   some_string.push_str(", world");
// }



pub fn main() {
  let mut s = String::from("hello");

  change(&mut s);

  println!("{}", s)
}

fn change(some_string: &mut String) {
  some_string.push_str(", rust!");
}