use std::io::{self, Write};
use lab4::*;
use csv;

fn main() {
    let n = {
        let mut input = String::new();
        print!("Enter the number of points (0 < N < 256): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please enter a valid number")
    };

    if n <= 1 || n >= 256 {
        println!("N must be between 1 and 255.");
        return;
    }

    let points = {
        let mut points = Vec::new();
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(io::stdin());
        let mut count = 0;

        for result in rdr.records() {
            let record = result.expect("Invalid csv format");

            let x: f32 = record[0].parse().expect("Invalid x value");
            let y: f32 = record[1].parse().expect("Invalid y value");

            points.push(Vec2d { x, y });
            count += 1;

            if count >= n {
                break;
            }
        }

        points
    };

    println!("{:?}", evaluate_rotations(&points));
}
