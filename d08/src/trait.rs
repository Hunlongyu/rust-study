trait Area {
  fn area(&self) -> f64;
}

struct Cicle {
  radius: f64,
}

struct Rectangle {
  width: f64,
  height: f64,
}

impl Area for Cicle {
  fn area(&self) -> f64 {
      use std::f64::consts::PI;
      PI * self.radius.powf(2.0)
  }
}

impl Area for Rectangle {
  fn area(&self) -> f64 {
      self.width * self.height
  }
}

fn main () {
  let circle = Cicle { radius: 5.0 };
  println!("Circle area: {}", circle.area());

  let rectangle = Rectangle { width: 10.0, height: 2.0 };
  println!("Rectangle area: {}", rectangle.area());
}
