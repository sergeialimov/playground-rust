fn no_dangle() -> String {
  let s = String::from("hello");

  s
}

pub fn main() {
  let _reference_to_nothing = no_dangle();
}