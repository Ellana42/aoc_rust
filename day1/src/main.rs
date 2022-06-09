use std::fs;

const FILENAME: &str = "input";

fn to_int(s: &str) -> i32 {
    s.parse().unwrap()
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let input = input.trim().split("\n");
    let input: Vec<i32> = input.map(to_int).collect();
    let mut increases = 0;
    let mut previous = input[0];
    for number in input {
        if number > previous {
            increases = increases + 1;
        }
        previous = number;
    }
    println!("{}", increases);
}
