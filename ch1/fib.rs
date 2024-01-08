//피보나치 수열 구하기

fn main() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);

    for i in 0..50 {
        println!("{}: {}", i + 2, a + b);
        let tmp = a;
        a = b;
        b = tmp + b;
    }
}
