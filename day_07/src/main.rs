use std::collections::{HashMap, VecDeque};

fn get_all_permutations(vector: Vec<i64>) -> Vec<Vec<i64>> {
    let mut ret: Vec<Vec<i64>> = Vec::new();

    if vector.len() == 0 {
        return Vec::new();
    } else if vector.len() == 1 {
        return vec![vector];
    }

    for (i, val) in vector.iter().enumerate() {
        let slice1: &[i64] = &vector[..i];
        let slice2: &[i64] = &vector[i + 1..];

        let mut left_over = slice1.to_vec();
        left_over.extend_from_slice(slice2);
        for p in get_all_permutations(left_over).iter() {
            let mut new_vec: Vec<i64> = vec![*val];
            new_vec.extend(p);
            ret.push(new_vec);
        }
    }

    ret
}

fn int_computer(instructions: &mut Vec<i64>, inputs: &mut VecDeque<i64>, i_ptr: &mut usize) -> i64 {
    while *i_ptr < instructions.len() {
        let opcode = instructions[*i_ptr] % 100;
        let p1 = (instructions[*i_ptr] / 100) % 10;
        let p2 = (instructions[*i_ptr] / 1000) % 10;
        let _p3 = (instructions[*i_ptr] / 10000) % 10;

        if opcode == 1 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = instructions[*i_ptr + 3] as usize;

            instructions[n3] = n1 + n2;

            *i_ptr += 4;
        } else if opcode == 2 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = instructions[*i_ptr + 3] as usize;

            instructions[n3] = n1 * n2;

            *i_ptr += 4;
        } else if opcode == 3 {
            let n1 = instructions[*i_ptr + 1 as usize] as usize;
            instructions[n1] = inputs.pop_front().unwrap();
            *i_ptr += 2;
        } else if opcode == 4 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            *i_ptr += 2;
            return n1;
        //i += 2;
        } else if opcode == 5 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };

            if n1 != 0 {
                *i_ptr = n2 as usize;
            } else {
                *i_ptr += 3;
            }
        } else if opcode == 6 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };

            if n1 == 0 {
                *i_ptr = n2 as usize;
            } else {
                *i_ptr += 3;
            }
        } else if opcode == 7 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = instructions[*i_ptr + 3] as usize;

            if n1 < n2 {
                instructions[n3] = 1;
            } else {
                instructions[n3] = 0;
            };

            *i_ptr += 4;
        } else if opcode == 8 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = instructions[*i_ptr + 3] as usize;

            if n1 == n2 {
                instructions[n3] = 1;
            } else {
                instructions[n3] = 0;
            };

            *i_ptr += 4;
        } else if opcode == 99 {
            //println!("break!");
            //break;
            return -1;
        }
    }

    0
}

fn seven_dot_two() -> () {
    // Test case 1
    //let instructions = vec![
    //    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    //];
    //let phases: Vec<i64> = vec![4, 3, 2, 1, 0];

    // Test case 2
    //let instructions = vec![
    //    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
    //    0, 0,
    //];
    //let phases: Vec<i64> = vec![0, 1, 2, 3, 4];

    // Test case 3
    //let instructions = vec![
    //    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
    //    31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    //];
    //let phases: Vec<i64> = vec![0, 1, 2, 3, 4];

    // Test case 1.2
    //let phases: Vec<i64> = vec![9, 8, 7, 6, 5];
    //let instructions = vec![
    //    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
    //    1005, 28, 6, 99, 0, 0, 5,
    //];

    // Test case 2.2
    //let phases: Vec<i64> = vec![9, 7, 8, 5, 6];
    //let instructions = vec![
    //    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
    //    54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
    //    1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    //];

    // Test case 3.2

    // Real input
    let instructions = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 30, 55, 76, 97, 114, 195, 276, 357, 438, 99999, 3,
        9, 102, 3, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 3, 9, 1001, 9, 5, 9, 1002, 9, 2, 9, 1001, 9, 2,
        9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 1001, 9, 2, 9, 102, 5, 9, 9, 1001, 9, 4, 9,
        4, 9, 99, 3, 9, 1001, 9, 4, 9, 102, 5, 9, 9, 101, 4, 9, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9,
        101, 2, 9, 9, 102, 4, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4,
        9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
        101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4,
        9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002,
        9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9,
        4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2,
        9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001,
        9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99,
    ];

    // Part 1
    //let phases: Vec<i64> = vec![0, 1, 2, 3, 4];
    // Part 2
    let phases: Vec<i64> = vec![5, 6, 7, 8, 9];
    let phase_it = get_all_permutations(phases);

    let mut total_max_output = i64::min_value();
    for phases in phase_it {
        let mut instructions_map: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut i_ptr_map: HashMap<i64, usize> = HashMap::new();
        let mut inputs_map: HashMap<i64, VecDeque<i64>> = HashMap::new();
        for phase in phases.iter() {
            instructions_map.insert(*phase, instructions.clone());
            i_ptr_map.insert(*phase, 0);
            inputs_map.insert(*phase, VecDeque::new());
            inputs_map.get_mut(phase).unwrap().push_back(*phase);
        }

        let mut input = 0;
        let mut max_output = i64::min_value();
        while input != -1 {
            for phase in phases.iter() {
                inputs_map.get_mut(phase).unwrap().push_back(input);
                let program = instructions_map.get_mut(phase).unwrap();
                let i_ptr = i_ptr_map.get_mut(phase).unwrap();
                let output = int_computer(program, inputs_map.get_mut(phase).unwrap(), i_ptr);
                if output > max_output {
                    max_output = output;
                }
                input = output;
                if input == -1 {
                    break;
                }
            }
        }
        if max_output > total_max_output {
            total_max_output = max_output;
        }
    }
    println!("{}", total_max_output);
}

fn main() {
    seven_dot_two();
}
