#[derive(Debug)]
struct Foo {
  value: usize,
}

fn reverse_and_print(foo: &mut Vec<Foo>) {
  for f in foo.iter_mut().rev() {
    println!("{:?}", f.value);
    f.value = 5;
    println!("{:?}", f.value);
  }
}

pub fn main () {
  let mut vector = vec![Foo {value: 1}, Foo {value: 2}, Foo {value: 3}];

  reverse_and_print(&mut vector);
  reverse_and_print(&mut vector);
}
