// use std::{mem, u8};

pub fn main () {
  let a: [u8; 5] = [1, 2, 3, 4, 5 ];
  
  // for item in a {
  //   println!("{}", item);
  // }

  // slice
  let b = &a[1 .. 4];
  for item in b {
    println!("{}", item);
  }

  assert_eq!([2, 3, 4], b);


  // println!("Array occupies {} bytes", mem::size_of_val(&a));
}