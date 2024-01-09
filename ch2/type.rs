use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let a = 100u8;
    let b = 100i128;
    let c = 10_000;
    println!(
        "Check each types: a={}({}), b={}({}), c={}({})",
        a,
        type_of(a),
        b,
        type_of(b),
        c,
        type_of(c),
    );

    println!("\ncheck char type");

    let a = 'a'; //문자 'a' 지정
    let b = b'a'; // ASCII 코드 97u8을 지정
    let c = '\x61'; // 16진수로 문자 'a'를 지정
    println!("{}, {:2x}, {}", a, b, c);

    let d = '곰'; //문자 '곰'을 지정
    let e = '곰' as u32; //문자 '곰'의 문자코드 'acf0'를 지정
    let f = '\u{acf0}'; //16진수로 문자 '곰' 지정
    println!("{},{:4x},{}", d, e, f);

    println!("\ncheck hex_oct");
    let v1 = 0xFF; //16진수로 255를 지정
    let v2 = 0o655; //8진수로 429를 지정
    let v3 = 0b1101_0101; //2진수로 213을 지정
    println!("{},{},{}", v1, v2, v3);

    println!("\ncheck float type");
    let f1 = 10.5; //부동 소수점 숫자 리터럴
    let f2 = 10.5f32; //f32타입 부동 소수점
    let f3 = 10.5e+8; //지수 형식으로 10500000000을 지정
    println!("{},{},{}", f1, f2, f3);
}
