fn main() {
    // add string type  =
    let s = "365";
    //parse to i32
    let i: i32 = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };

    println!("{}", i);
}
