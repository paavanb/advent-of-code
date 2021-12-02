use std::fs;


struct Submarine {
    position: i32,
    depth: i32,
    aim: i32,
}

struct Instruction {
    op: String,
    units: i32,
}

fn main() {
    let sub_one = part_one();
    let sub_two = part_two();
    println!("Part one: {}", sub_one.position * sub_one.depth);
    println!("Part two: {}", sub_two.position * sub_two.depth);
}


fn part_one() -> Submarine {
    let loc = Submarine {
        position: 0,
        depth: 0,
        aim: 0,
    };

    let instructions = get_instructions();

    return instructions.iter().fold(loc, |accum_loc, ixn| {
        let position_offset = match ixn.op.as_str() {
            "forward" => ixn.units,
            _ => 0,
        };
        let depth_offset = match ixn.op.as_str() {
            "up" => -ixn.units,
            "down" => ixn.units,
            _ => 0,
        };

        Submarine {
            position: accum_loc.position + position_offset,
            depth: accum_loc.depth + depth_offset,
            aim: 0,
        }
    });
}


fn part_two() -> Submarine {
    let loc = Submarine {
        position: 0,
        depth: 0,
        aim: 0,
    };

    let instructions = get_instructions();

    return instructions.iter().fold(loc, |accum_loc, ixn| {
        match ixn.op.as_str() {
            "forward" => Submarine {
                position: accum_loc.position + ixn.units,
                depth: accum_loc.depth + accum_loc.aim * ixn.units,
                aim: accum_loc.aim,
            },
            "up" => Submarine {
                position: accum_loc.position,
                depth: accum_loc.depth,
                aim: accum_loc.aim - ixn.units,
            },
            "down" => Submarine {
                position: accum_loc.position,
                depth: accum_loc.depth,
                aim: accum_loc.aim + ixn.units,
            },
            _ => accum_loc,
        }
    });
}


/**
 * Grab instructions from puzzle input file.
 */
fn get_instructions() -> Vec<Instruction> {
    let contents = fs::read_to_string("data/submarine_instructions.txt")
        .expect("Something went wrong.");
    let instructions: Vec<Instruction> = contents
        .trim_end()
        .split("\n")
        .map(|s| {
            let mut parts: Vec<&str> = s.split(' ').collect();
            assert_eq!(parts.len(), 2, "Expected instruction to consist of two parts.");

            let units = parts.pop().expect("units").parse::<i32>().expect("Invalid number.");
            let op = parts.pop().expect("operation");
            Instruction {
                op: str::to_string(op),
                units,
            }
        })
        .collect();

    return instructions;
}
