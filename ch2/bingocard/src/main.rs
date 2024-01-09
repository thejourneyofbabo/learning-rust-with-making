use rand::seq::SliceRandom;

fn main() {
    //make array range 1~75
    let mut nums = [0; 75];
    for i in 1..=75 {
        nums[i - 1] = i;
    }

    //mixing
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    //show card
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                //wild card
                print!(" *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
