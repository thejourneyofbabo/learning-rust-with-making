use rand::Rng;

//전체 미로의 크기 지정
const MAP_N: usize = 25;

fn main() {
    //난수 생성기 준비
    let mut rng = rand::thread_rng();
    // 미로 초기화
    let mut maze = [[0; MAP_N]; MAP_N];
    //둘레를 벽으로 감싸기
    for n in 0..MAP_N {
        maze[n][0] = 1; // 위쪽 벽 만들기
        maze[0][n] = 1; // make leftside wall
        maze[n][MAP_N - 1] = 1; //make rightside wall
        maze[MAP_N - 1][n] = 1; //make botton wall
    }
    //every two spot, one wall
    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1; // wall
                            // 상하좌우 중 어느 하나를 벽으로 만들기
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y - 1][x] = 1, // up
                1 => maze[y + 1][x] = 1, // down
                2 => maze[y][x - 1] = 1, // left
                3 => maze[y][x + 1] = 1, // right
                _ => {}
            }
        }
    }
    // printing maze
    // 0과 1을 각각 흰색 타일(U+2B1C)과 검은색 타일(U+2B1B)로 치환한다
    let tiles = ["⬜️", "⬛️"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
