use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let data: i64 = line.unwrap().trim().parse().unwrap();
        println!("{}", data);
    }
}
