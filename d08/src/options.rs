struct Point<T> {
  x: T,
  y: T,
}

struct Test<T, U> {
  a: T,
  b: U,
}

fn main() {
  let boolean = Point { x: true, y: false };
  let integer = Point { x: 1, y: 0 };
  let float = Point { x: 1.1, y: 4.5 };
  let string_slice = Point { x: "high", y: "low" };

  let test1 = Test { a: 1, b: true };
}
