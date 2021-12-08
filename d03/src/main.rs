struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}


#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}

fn main () {
    let mut car = car_factory(String::from("black"), Transmission::Manual, false);
    println!("car 1 = {}, {:?} transimission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("red"), Transmission::SemiAuto, true);
    println!("car 2 = {}, {:?} transimission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("blue"), Transmission::Automatic, false);
    println!("car 3 = {}, {:?} transimission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}
