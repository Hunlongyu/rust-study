#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles)
    }
    (Age::New, miles)
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }
    Car { color, motor, roof, age: car_quality(miles) }
}

fn main() {

    let colors = ["blue", "Green", "Red", "Sliver"];

    let mut engine = Transmission::Manual;

    car_factory(String::from(colors[0]), engine, true, 0);

    engine = Transmission::SemiAuto;
    car_factory(String::from(colors[1]), engine, false, 100);

    engine = Transmission::Automatic;
    car_factory(String::from(colors[2]), engine, true, 200);

    let days = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
    let bytes = [0; 5];
    println!("day: {}", days[1]);
    println!("byte: {}", bytes[2]);

    let nums = vec![12, 3, 54];
    println!("vector: {:?}", nums);

    let zeros = vec![1; 5];
    println!("zeros: {:?}", zeros);

    let mut fruit = Vec::new();
    fruit.push("apple");
    fruit.push("banana");
    fruit[1] = "mango";
    println!("fruit: {:?}", fruit);
}
