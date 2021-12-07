fn main() {
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    let x = 7;
    println!("{}", x);

    const MAX_POINTS: i32 = 100_000;
    let str = "la la la";
    println!("{}, {}", MAX_POINTS, str.len());

    let num: u32 = "42".parse().expect("Not a number");
    println!("{}", num);

    let str: u32 = match "a".parse() {
        Ok(num) => num,
        Err(_) => 2
    };
    println!("{}", str);

    let _a = 3.4;
    let _b: f32 = 3.5;
    let _c = true;

    let tup: (i32, f64, u8) = (500, 3.2, 7);
    let (d, e, f) = tup;
    println!("{},{},{}", d, e, f);
    println!("{}{}{}", tup.0, tup.1, tup.2);

    let arr = ["a", "b", "c", "d"];
    println!("{}", arr[2]);

    another_fun("hly".to_string());

    println!("{}", plus_five(6));

    for item in arr.iter() {
        println!("{}", item);
    }
}

fn another_fun (params: String) {
    println!("welcome: {}", params);
    for n in (1..5).rev() {
        println!("{}", n)
    }
    let flag = true;
    if flag {
        println!("{}", flag)
    } else {
        println!("{}", flag)
    }

    let bl = if flag { 5 } else { 6 };
    let nl = if flag { "abc" } else { "def" };
    println!("{}, {}", bl, nl);
}

fn plus_five (x: i32) -> i32 {
   x + 5
}