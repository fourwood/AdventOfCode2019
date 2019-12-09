use std::collections::VecDeque;

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

fn int_computer(instructions: &mut Vec<i64>, inputs: &mut VecDeque<i64>,
                i_ptr: &mut usize, rel_base: &mut i64) -> i64 {
    while *i_ptr < instructions.len() {
        let opcode = instructions[*i_ptr] % 100;
        let p1 = (instructions[*i_ptr] / 100) % 10;
        let p2 = (instructions[*i_ptr] / 1000) % 10;
        let p3 = (instructions[*i_ptr] / 10000) % 10;

        if opcode == 1 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else if p2 == 2 {
                instructions[(instructions[*i_ptr + 2] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = if p3 == 2 {
                (instructions[*i_ptr + 3] + *rel_base) as usize
            } else {
                instructions[*i_ptr + 3] as usize
            };

            instructions[n3] = n1 + n2;

            *i_ptr += 4;
        } else if opcode == 2 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else if p2 == 2 {
                instructions[(instructions[*i_ptr + 2] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = if p3 == 2 {
                (instructions[*i_ptr + 3] + *rel_base) as usize
            } else {
                instructions[*i_ptr + 3] as usize
            };

            instructions[n3] = n1 * n2;

            *i_ptr += 4;
        } else if opcode == 3 {
            let n1 = if p1 == 2 {
                (instructions[*i_ptr + 1 as usize] + *rel_base) as usize
            } else {
                instructions[*i_ptr + 1 as usize] as usize
            };
            instructions[n1] = inputs.pop_front().unwrap();
            *i_ptr += 2;
        } else if opcode == 4 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            *i_ptr += 2;
            //println!("output: {}", n1);
            return n1;
        //i += 2;
        } else if opcode == 5 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else if p2 == 2 {
                instructions[(instructions[*i_ptr + 2] + *rel_base) as usize]
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
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else if p2 == 2 {
                instructions[(instructions[*i_ptr + 2] + *rel_base) as usize]
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
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else if p2 == 2 {
                instructions[(instructions[*i_ptr + 2] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = if p3 == 2 {
                (instructions[*i_ptr + 3] + *rel_base) as usize
            } else {
                instructions[*i_ptr + 3] as usize
            };

            if n1 < n2 {
                instructions[n3] = 1;
            } else {
                instructions[n3] = 0;
            };

            *i_ptr += 4;
        } else if opcode == 8 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[*i_ptr + 2 as usize]
            } else if p2 == 2 {
                instructions[(instructions[*i_ptr + 2] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 2] as usize]
            };
            let n3 = if p3 == 2 {
                (instructions[*i_ptr + 3] + *rel_base) as usize
            } else {
                instructions[*i_ptr + 3] as usize
            };

            if n1 == n2 {
                instructions[n3] = 1;
            } else {
                instructions[n3] = 0;
            };

            *i_ptr += 4;
        } else if opcode == 9 {
            let n1 = if p1 == 1 {
                instructions[*i_ptr + 1 as usize]
            } else if p1 == 2 {
                instructions[(instructions[*i_ptr + 1] + *rel_base) as usize]
            } else {
                instructions[instructions[*i_ptr + 1] as usize]
            };

            *rel_base += n1;

            *i_ptr += 2;
        } else if opcode == 99 {
            //println!("break!");
            //break;
            return i64::min_value();
        }
    }

    0
}

fn nine_dot_two() -> () {
    // Test case one
    //let mut program: Vec<i64> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    //program.append(&mut vec![0; 100]);
    //let mut input: VecDeque<i64> = VecDeque::new();

    // Test case two
    //let mut program: Vec<i64> = vec![1102,34915192,34915192,7,4,7,99,0];
    //let mut input: VecDeque<i64> = VecDeque::new();

    let mut program: Vec<i64> = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,37,0,1005,1101,30,0,1013,1102,1,33,1019,1102,1,25,1003,1102,1,28,1018,1101,26,0,1006,1102,1,866,1029,1101,760,0,1023,1102,39,1,1012,1102,23,1,1009,1101,281,0,1026,1102,1,20,1011,1102,1,34,1008,1101,0,36,1017,1101,38,0,1000,1102,0,1,1020,1102,278,1,1027,1101,21,0,1010,1102,875,1,1028,1101,0,212,1025,1102,1,1,1021,1102,1,24,1014,1102,763,1,1022,1101,0,31,1007,1102,1,221,1024,1101,0,32,1002,1102,1,29,1004,1102,1,35,1016,1102,22,1,1015,1101,0,27,1001,109,9,1207,-6,26,63,1005,63,199,4,187,1105,1,203,1001,64,1,64,1002,64,2,64,109,19,2105,1,-4,4,209,1001,64,1,64,1106,0,221,1002,64,2,64,109,-33,1207,5,37,63,1005,63,241,1001,64,1,64,1106,0,243,4,227,1002,64,2,64,109,16,2102,1,-2,63,1008,63,23,63,1005,63,269,4,249,1001,64,1,64,1106,0,269,1002,64,2,64,109,16,2106,0,0,1106,0,287,4,275,1001,64,1,64,1002,64,2,64,109,-11,21101,40,0,0,1008,1016,38,63,1005,63,311,1001,64,1,64,1105,1,313,4,293,1002,64,2,64,109,4,21107,41,40,-9,1005,1011,329,1105,1,335,4,319,1001,64,1,64,1002,64,2,64,109,-14,21108,42,42,5,1005,1011,353,4,341,1106,0,357,1001,64,1,64,1002,64,2,64,109,2,2107,33,0,63,1005,63,379,4,363,1001,64,1,64,1105,1,379,1002,64,2,64,109,-7,1201,2,0,63,1008,63,25,63,1005,63,401,4,385,1105,1,405,1001,64,1,64,1002,64,2,64,109,11,1201,-8,0,63,1008,63,28,63,1005,63,429,1001,64,1,64,1106,0,431,4,411,1002,64,2,64,109,-7,2108,26,1,63,1005,63,449,4,437,1105,1,453,1001,64,1,64,1002,64,2,64,109,9,1206,7,465,1105,1,471,4,459,1001,64,1,64,1002,64,2,64,109,4,21102,43,1,-3,1008,1015,42,63,1005,63,491,1106,0,497,4,477,1001,64,1,64,1002,64,2,64,109,7,21108,44,43,-7,1005,1018,517,1001,64,1,64,1105,1,519,4,503,1002,64,2,64,109,-28,2101,0,7,63,1008,63,29,63,1005,63,545,4,525,1001,64,1,64,1105,1,545,1002,64,2,64,109,11,2107,28,-7,63,1005,63,561,1105,1,567,4,551,1001,64,1,64,1002,64,2,64,109,-4,2101,0,-1,63,1008,63,26,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,9,1206,7,607,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-10,1208,1,27,63,1005,63,627,1106,0,633,4,617,1001,64,1,64,1002,64,2,64,109,26,1205,-9,649,1001,64,1,64,1106,0,651,4,639,1002,64,2,64,109,-20,1208,0,23,63,1005,63,669,4,657,1105,1,673,1001,64,1,64,1002,64,2,64,109,-7,2102,1,1,63,1008,63,28,63,1005,63,693,1105,1,699,4,679,1001,64,1,64,1002,64,2,64,109,18,21102,45,1,-6,1008,1014,45,63,1005,63,725,4,705,1001,64,1,64,1106,0,725,1002,64,2,64,109,-23,1202,6,1,63,1008,63,25,63,1005,63,751,4,731,1001,64,1,64,1106,0,751,1002,64,2,64,109,20,2105,1,6,1106,0,769,4,757,1001,64,1,64,1002,64,2,64,109,-22,2108,39,10,63,1005,63,789,1001,64,1,64,1106,0,791,4,775,1002,64,2,64,109,3,1202,6,1,63,1008,63,32,63,1005,63,815,1001,64,1,64,1105,1,817,4,797,1002,64,2,64,109,23,21107,46,47,-9,1005,1012,835,4,823,1106,0,839,1001,64,1,64,1002,64,2,64,109,1,1205,-1,853,4,845,1105,1,857,1001,64,1,64,1002,64,2,64,109,-2,2106,0,8,4,863,1001,64,1,64,1105,1,875,1002,64,2,64,109,-8,21101,47,0,-2,1008,1010,47,63,1005,63,897,4,881,1106,0,901,1001,64,1,64,4,64,99,21102,27,1,1,21101,0,915,0,1105,1,922,21201,1,27810,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1106,0,968,22101,0,-2,-2,109,-3,2106,0,0];
    program.append(&mut vec![0; 1000]);
    let mut input: VecDeque<i64> = VecDeque::new();
    // Input part 1
    //input.push_back(2);
    // Input part 2
    input.push_back(2);

    let mut i_ptr: usize = 0;
    let mut rel_base: i64 = 0;
    let mut output = 0;
    while output != i64::min_value() {
        output = int_computer(&mut program, &mut input, &mut i_ptr, &mut rel_base);

        if output == i64::min_value() {
            break;
        } else {
            println!("{}", output);
        }
    }
}

fn main() {
    nine_dot_two();
}
