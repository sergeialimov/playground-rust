pub fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);
  // println!("{}", s1);  // here would be an error — value borrowed here after move
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
