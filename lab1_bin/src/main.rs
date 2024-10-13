use lab1::algorithms::*;
use lab1::containers::ListBasedQueue;
use common::containers::traits::FillableContainer;

fn option_to_str<T: std::fmt::Display>(opt: &Option<T>) -> String {
    if let Some(t) = opt {
        t.to_string()
    } else {
        "None".to_string()
    }
}

fn main() {
    const MAX_NUMBERS: usize = 256;

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Failed to parse N");

    if n > MAX_NUMBERS {
        println!("Input number {} exceeds maximum {}.", n, MAX_NUMBERS);
        return;
    }

    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    if numbers.len() != n {
        println!("Expected {} numbers, but got {}", n, numbers.len());
        return;
    }

    let produce_queue = || {
        let mut queue = ListBasedQueue::new();

        for number in &numbers {
            queue.push(*number);
        }

        queue
    };

    let sum = sum(produce_queue());
    let mean = mean(produce_queue());
    let (mins, maxs) = evaluate_mins_and_maxs_triples(produce_queue());
    let central_element = central_element(produce_queue());

    println!("{}", sum);
    println!("{}", option_to_str(&mean));
    println!(
        "{} {} {}",
        option_to_str(&mins.triple.first),
        option_to_str(&mins.triple.second),
        option_to_str(&mins.triple.third)
    );
    println!(
        "{} {} {}",
        option_to_str(&maxs.triple.first),
        option_to_str(&maxs.triple.second),
        option_to_str(&maxs.triple.third)
    );
    println!("{}", option_to_str(&central_element))
}
