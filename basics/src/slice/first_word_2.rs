pub fn first_word (s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item)in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

pub fn main () {
  let str = String::from("split it");
  // let res = first_word(&str[0..5]);
  // let res = first_word(&str[..]);
  // let res = first_word(&str);
  let res = first_word(&str);

  let str2 = "hey rust!";
  let res2 = first_word(str2);


  println!("{}", res);
  println!("{}", res2);
}

