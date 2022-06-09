use std::fs;

struct BinRates {
    gamma: Vec<i32>,
    epsilon: Vec<i32>,
}

fn convert(binary: Vec<i32>) -> i32 {
    let size = binary.len();
    let mut result = 0;
    for i in 0..size {
        result = result + binary[i] * 2_i32.pow(u32::try_from(size - i - 1).unwrap());
    }
    result
}

fn main() {
    let instructions = fs::read_to_string("input").expect("Couldn't read the subject");
    let mut table_inst: Vec<Vec<u32>> = Vec::new();
    for instruction in instructions.lines() {
        table_inst.push(
            instruction
                .chars()
                .map(|s| s.to_digit(10).unwrap())
                .collect(),
        );
    }
    let len = table_inst.len();
    let width = table_inst[0].len();
    let mut sum_table: Vec<i32> = Vec::new();
    let mut counter: i32;
    for i in 0..width {
        counter = 0;
        for j in 0..len {
            counter = counter + table_inst[j][i] as i32;
        }
        sum_table.push(counter);
    }
    let bin_rates = BinRates {
        gamma: sum_table
            .iter()
            .map(|x| if x * 2 > len as i32 { 1 } else { 0 })
            .collect(),
        epsilon: sum_table
            .iter()
            .map(|x| if x * 2 > len as i32 { 0 } else { 1 })
            .collect(),
    };
    let gamma = convert(bin_rates.gamma);
    let epsilon = convert(bin_rates.epsilon);
    println!("Gamma : {} - Epsilon : {}", gamma, epsilon);
    println!(" Result {} ", gamma * epsilon);
}
