pub fn calculate_price_of_apples(num: u32) -> u32 {
  let price = if num > 40 {
      1
  } else {
      2
  };

  num * price
}