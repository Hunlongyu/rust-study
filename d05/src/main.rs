use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(String::from("Ancient"), String::from("very accurate."));
    reviews.insert(String::from("hly"), String::from("dd"));

    let book: &str = "Ancient";
    reviews.remove(&"Ancient".to_string());
    println!("\n Review for \'{}\': {:?}", book, reviews.get(book));

    let mut conter = 1;
    let stop_loop = loop {
        conter *= 2;
        if conter > 100 {
            break conter;
        }
    };
    println!("{}", stop_loop);

    let mut idx = 1;
    while idx < 5 {
        idx += 1;
        println!("{}", idx)
    }
    println!("idx: {}", idx);

    let birds = ["ostrich", "peacock", "stork"];
    for bird in birds.iter() {
        println!("{}", bird);
    }

    for num in 0..5 {
        println!("{}", num);
    }
}
