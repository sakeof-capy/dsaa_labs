use lab2::PersonArrayMap;
use std::io::{self, BufRead};

fn main() {
    let mut map = PersonArrayMap::default();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines
        .next()
        .expect("Failed to read line")
        .expect("Failed to parse line");
    let mut parts = first_line.split_whitespace();
    let m: usize = parts
        .next()
        .expect("Missing m value")
        .parse()
        .expect("Failed to parse m");
    let n: usize = parts
        .next()
        .expect("Missing n value")
        .parse()
        .expect("Failed to parse n");

    println!("m = {}, n = {}", m, n);

    let mut key_value_pairs = Vec::with_capacity(m);
    let mut query_keys = Vec::with_capacity(n);

    for _ in 0..m {
        if let Some(Ok(line)) = lines.next() {
            let mut kv = line.split_whitespace();
            let key = kv.next().expect("Missing key").to_string();
            let value = kv.next().expect("Missing value").to_string();
            key_value_pairs.push((key, value));
        }
    }

    for _ in 0..n {
        if let Some(Ok(key)) = lines.next() {
            query_keys.push(key);
        }
    }

    for (key, value) in key_value_pairs {
        map.put(key, value);
    }

    println!("-------------------------");

    for key in query_keys {
        match map.get(&key) {
            Some(value) => println!("{}", value),
            None => println!("Key not found"),
        }
    }
}

/*
3 2
England London
France Paris
Germany Berlin
France
England
*/
