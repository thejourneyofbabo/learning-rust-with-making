use std::io;

fn input_str() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Faild to read line");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}

fn main() {
    let mut height;

    loop {
        println!("height(cm): ");
        height = input_f(0.0); //write number
        if height > 0.0 {
            break;
        }
        println!("Please input valid number")
    }
    //show standard weight
    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("Standard weight is: {:.1}", weight);
}
