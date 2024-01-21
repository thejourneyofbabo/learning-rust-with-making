use std::env; //명령줄 인수 취득용
use std::fs; //파일 읽기용

fn main() {
    //인수를 벡터로 취득
    let args: Vec<String> = env::args().collect();
    //인수를 지정했는지 확인
    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요");
        return;
    }
    // 두 번째 요소
    let filename = &args[1];
    //파일을 읽어와 출력
    //let text = fs::read_to_string(filename).unwrap();

    let text = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    println!("{}", text);
}
