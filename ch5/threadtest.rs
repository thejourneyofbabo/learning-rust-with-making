use std::{thread, time};

fn sleep_print(word: &str) {
    for i in 1..=3 {
        println!("{}: i={}", word, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    //without thread
    println!("---without Thread---");
    sleep_print("no thread");

    //using thread
    println!("---using Thread---");
    //Thread1
    thread::spawn(|| sleep_print("Swiss"));
    //Thread2
    thread::spawn(|| sleep_print("Tomato"));
    //Thread3
    thread::spawn(|| sleep_print("Shooting Stars"));
    //main thread
    sleep_print("Jisang is the Best");
}
