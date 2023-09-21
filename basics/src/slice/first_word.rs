pub fn first_word (s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item)in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

pub fn main () {
  let str: &String = &String::from("split it");
  let res = first_word(str);
  println!("{}", res);
}

/*
In the given code snippet, b' ' represents a byte literal for the space character.

In Rust, String is a collection of bytes (Vec<u8>) that is guaranteed to be valid UTF-8. In the function first_word, s is a reference to a String, and s.as_bytes() converts the String to a byte slice (&[u8]) view of the String.

The loop then iterates over each byte (and its index) in the byte slice, checking whether the current byte item is equal to the byte literal for space (b' '). If it finds a space, it returns the index of that byte, effectively finding the index of the end of the first word in the string. If no space is found, it returns the length of the string, effectively treating the entire string as one word.
*/

/*
b is not exactly a keyword, but it is a prefix that you can use before a character or a string literal to create a byte (u8) or a byte array (&[u8]) respectively. For example, b'a' is a byte (u8) literal representing the ASCII value of 'a', and b"hello" is a byte array (&[u8]) literal representing the ASCII values of 'h', 'e', 'l', 'l', 'o'. This syntax is specific to Rust and may not be present or may have different semantics in other programming languages.
 */