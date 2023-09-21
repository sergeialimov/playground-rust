fn first_word (s: &String) -> usize {
  println!("{}", s);
  return 3;
}

pub fn main () {
  let str: &String = &String::from("split it");
  let res = first_word(str);
  println!("{}", res);
}