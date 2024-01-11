use rand::Rng;
use std::collections::HashMap;
use std::io;

fn input_str(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Faild to read line");
    input.trim().to_string()
}

fn input_int(prompt: &str) -> i32 {
    loop {
        let input = input_str(prompt);
        match input.parse() {
            Ok(v) => return v,
            Err(_) => println!("Please input valid number"),
        }
    }
}

fn count(data: Vec<&str>) {
    println!("{:?}", data);
    let mut c_map = HashMap::new();

    for k in &data {
        let count = c_map.entry(k).or_insert(0);
        *count += 1;
    }

    for (key, value) in c_map.iter() {
        println!("{}: {:>2}", key, value);
    }
}
fn main() {
    let mut rand_data = vec![];

    //   println!("How many rand case you'd like to make?: ");
    let num = input_int("How many random case would you like to generate?: ");
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let r = rng.gen_range(0..3);
        match r {
            0 => rand_data.push("A"),
            1 => rand_data.push("B"),
            2 => rand_data.push("C"),
            _ => panic!("Unexpected value from random generator"),
        }
    }

    count(rand_data);
}
