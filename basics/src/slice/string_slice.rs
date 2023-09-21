pub fn main () {
  let s = String::from("hey rust");

  let hey = &s[0..3];
  let rust = &s[4..8];
  
  let hey2 = &s[..3];
  let rust2 = &s[4..];

  let hey_rust = &s[..];

  println!("{}", hey);
  println!("{}", rust);
  println!("---------");
  println!("{}", hey2);
  println!("{}", rust2);
  println!("---------");
  println!("{}", hey_rust);
}