use std::io;

fn main() {
    let height_cm = input("Height (cm): ");
    let weight = input("Weight: ");

    // Calculate BMI
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI = {:.1}", bmi);

    // Check health (비만도)
    let health = match (bmi * 10.0) as u32 {
        0..=185 => "Low fat",
        186..=230 => "Normal",
        231..=250 => "Overweight",
        251..=300 => "Obese (Class 1)",
        301..=350 => "Obese (Class 2)",
        _ => "Obese (Class 3)",
    };

    println!("Health: {}", health);
}

fn input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        // Handling potential errors when parsing the input
        match s.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please input a valid number."),
        }
    }
}
