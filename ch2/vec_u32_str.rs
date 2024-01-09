fn main() {
    //make vector u32
    let a_vec: Vec<u32> = vec![100, 200, 300];

    for i in a_vec {
        println!("{}", i);
    }

    // &str type vector
    let s_vec: Vec<&str> = vec!["dog", "cat", "chicken"];

    for i in s_vec {
        println!("{}", i);
    }
}
