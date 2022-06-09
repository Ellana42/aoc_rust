use std::fs;

const FILENAME: &str = "input";

fn sum_three(vec: &[u16]) -> u16 {
    vec[0] + vec[1] + vec[2]
}

fn main() {
    let measurements = fs::read_to_string(FILENAME).expect("Problem reading the file");
    let measurements: Vec<u16> = measurements.lines().map(|s| s.parse().unwrap()).collect();
    let mut summed_measurements: Vec<u16> = Vec::new();
    let size = measurements.len();
    for (index, _measurement) in measurements.iter().enumerate() {
        if index < size - 2 {
            summed_measurements.push(sum_three(&measurements[index..index + 3]))
        }
    }
    let mut results = 0;
    let mut previous = summed_measurements[0];
    for n in 1..summed_measurements.len() {
        if previous < summed_measurements[n] {
            results = results + 1;
        }
        previous = summed_measurements[n];
    }
    println!("Result : {}", results);
}
