#[derive(Debug)]
struct Foo {}

fn main () {
  let mut vector = vec![Foo{}, Foo{}, Foo{}];

  // vector.
  let last_foo = vector.last();
  println!("last foo {:?}", last_foo);


  vector.pop();
}