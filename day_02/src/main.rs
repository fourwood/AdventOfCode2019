fn process_instructions(mut instructions: Vec<u32>, n_vals: usize, out_pos: usize) -> u32 {
    let num_instructions = instructions.len() / n_vals;

    for i in 0..num_instructions {
        let opcode = instructions[i * n_vals];
        if opcode == 99 {
            break;
        }

        let n1 = instructions[instructions[i * n_vals + 1] as usize];
        let n2 = instructions[instructions[i * n_vals + 2] as usize];
        let out = instructions[i * n_vals + 3] as usize;
        if opcode == 1 {
            instructions[out] = n1 + n2;
        } else if opcode == 2 {
            instructions[out] = n1 * n2;
        }
    }

    instructions[out_pos]
}

fn main() {
    let initial_instructions = vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 6, 23, 2, 23, 13, 27,
        1, 27, 5, 31, 2, 31, 10, 35, 1, 9, 35, 39, 1, 39, 9, 43, 2, 9, 43, 47, 1, 5, 47, 51, 2, 13,
        51, 55, 1, 55, 9, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 10, 67, 71, 1, 71, 10, 75, 2, 75, 13,
        79, 2, 79, 13, 83, 1, 5, 83, 87, 1, 87, 6, 91, 2, 91, 13, 95, 1, 5, 95, 99, 1, 99, 2, 103,
        1, 103, 6, 0, 99, 2, 14, 0, 0,
    ];
    let num_vals_per_instruction = 4;

    let answer = 19690720;

    for i in 0..100 {
        for j in 0..100 {
            let mut instructions = initial_instructions.to_vec();
            instructions[1] = i;
            instructions[2] = j;
            let result = process_instructions(instructions, num_vals_per_instruction, 0);
            if result == answer {
                println!("{}", i * 100 + j);
            }
        }
    }
}
