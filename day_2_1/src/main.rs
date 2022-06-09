use std::fs;

enum Mvt {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Position {
    pos: i32,
    depth: i32,
}

fn move_boat(mvt: Mvt, mut position: Position) -> Position {
    match mvt {
        Mvt::Up(i) => position.depth = position.depth - i,
        Mvt::Down(i) => position.depth = position.depth + i,
        Mvt::Forward(i) => position.pos = position.pos + i,
    };
    position
}

fn main() {
    let instructions = fs::read_to_string("input").expect("Couldn't read the subject");
    let instructions = instructions.lines();
    let mut array_intructions: Vec<(&str, i32)> = Vec::new();
    for instruction in instructions {
        let split: Vec<&str> = instruction.split(" ").collect();
        array_intructions.push((split[0], split[1].parse().unwrap()));
    }
    let mut position = Position { pos: 0, depth: 0 };
    for instruction in &array_intructions {
        let mvt: Mvt = match instruction.0 {
            "up" => Mvt::Up(instruction.1),
            "down" => Mvt::Down(instruction.1),
            "forward" => Mvt::Forward(instruction.1),
            &_ => Mvt::Forward(0),
        };
        position = move_boat(mvt, position);
    }
    println!(
        "Final depth : {} - Final position : {}",
        position.depth, position.pos
    );
    println!("Result : {}", position.depth * position.pos);
}
