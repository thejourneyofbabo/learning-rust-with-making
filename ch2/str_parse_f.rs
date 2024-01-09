fn main() {
    let s = "3.1415";
    let num = s.parse::<f64>().expect("변환실패");
    println!("{:.2}", num);

    let s = "3.1415a";
    let num = s.parse::<f64>();
    match num {
        Ok(result) => println!("{:2}", result),
        Err(e) => println!("error occur. Because of: '{:?}'", e),
    }
}
