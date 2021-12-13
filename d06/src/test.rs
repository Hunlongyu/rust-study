fn main() {
  let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

  for &index in [0, 2, 10].iter() {
      match fruits.get(index) {
          Some(&"coconut") => println!("Coconuts are awesome!"),
          Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
          None => println!("There is no fruit! :("),
      }
  }

  let a_number:Option<u8> = Some(7);
  match a_number {
      Some(7) => println!("That's my luck number!"),
      _=> {},
  }
  if let Some(7) = a_number {
      println!("That's my luck number!");
  }

  let gift = Some("candy");
  assert_eq!(gift.unwrap(), "candy");

  let empty_gift: Option<&str> = None;
  empty_gift.expect("fruits are healthy");
}
