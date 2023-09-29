use std::collections::HashMap;

// fn basic_map () {
//   let mut scores = HashMap::new();

//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Yellow"), 50);

//   let item = scores.get("Blue");
//   print!("{:?}", item);
// }

// basic_map();

pub fn main () {
  object_map();
}


fn object_map () {
  #[derive(Debug, Clone)]
  struct TestStruct {
    a: String,
    b: String,
  }

  let test_struct = TestStruct {
    a: String::from("aaa"),
    b: String::from("bbb"),
  };

  println!("{:?}", test_struct.a);
  let mut struct_map = HashMap::new();

  struct_map.insert(test_struct.a.clone(), test_struct.clone());

  let res = struct_map.get("aaa");

  match res {
    Some(value) => {
      println!("{:?}", value);
      println!("{:?}", value.b);
    }
    None => {
      println!("Key not found");
    }
  }

}