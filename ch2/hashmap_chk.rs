use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    //check the key exist
    if map.get("D") == None {
        println!("There're no D");
    } else {
        println!("D = {}", map["D"]);
    }
}
