fn main() {
    // Real inputs:
    let mut instructions = vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 68, 5, 225, 1101, 71, 12, 225, 1,
        117, 166, 224, 1001, 224, -100, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223,
        224, 223, 1001, 66, 36, 224, 101, -87, 224, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224,
        224, 1, 223, 224, 223, 1101, 26, 51, 225, 1102, 11, 61, 224, 1001, 224, -671, 224, 4, 224,
        1002, 223, 8, 223, 1001, 224, 5, 224, 1, 223, 224, 223, 1101, 59, 77, 224, 101, -136, 224,
        224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 223, 224, 223, 1101, 11, 36, 225,
        1102, 31, 16, 225, 102, 24, 217, 224, 1001, 224, -1656, 224, 4, 224, 102, 8, 223, 223,
        1001, 224, 1, 224, 1, 224, 223, 223, 101, 60, 169, 224, 1001, 224, -147, 224, 4, 224, 102,
        8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1102, 38, 69, 225, 1101, 87, 42, 225, 2,
        17, 14, 224, 101, -355, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 224, 223,
        223, 1002, 113, 89, 224, 101, -979, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224,
        1, 224, 223, 223, 1102, 69, 59, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105,
        1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 7, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 329, 1001, 223, 1, 223,
        1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 344, 1001, 223, 1, 223, 1108, 226, 677,
        224, 102, 2, 223, 223, 1005, 224, 359, 1001, 223, 1, 223, 1107, 226, 677, 224, 1002, 223,
        2, 223, 1006, 224, 374, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1006,
        224, 389, 101, 1, 223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 101, 1,
        223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 419, 101, 1, 223, 223, 1008,
        226, 226, 224, 102, 2, 223, 223, 1006, 224, 434, 101, 1, 223, 223, 107, 226, 226, 224,
        1002, 223, 2, 223, 1005, 224, 449, 1001, 223, 1, 223, 108, 226, 677, 224, 102, 2, 223, 223,
        1005, 224, 464, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 479,
        101, 1, 223, 223, 1007, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223,
        107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 509, 101, 1, 223, 223, 108, 677, 677, 224,
        102, 2, 223, 223, 1006, 224, 524, 1001, 223, 1, 223, 8, 226, 677, 224, 102, 2, 223, 223,
        1005, 224, 539, 101, 1, 223, 223, 107, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 554,
        1001, 223, 1, 223, 8, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 569, 1001, 223, 1, 223,
        7, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 584, 1001, 223, 1, 223, 1108, 226, 226,
        224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 1107, 677, 677, 224, 1002, 223,
        2, 223, 1006, 224, 614, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006,
        224, 629, 1001, 223, 1, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 644, 1001,
        223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 659, 1001, 223, 1, 223, 1008,
        677, 677, 224, 1002, 223, 2, 223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ];

    //let mut instructions = vec![1101, 100, -1, 4, 0];

    //let mut instructions = vec![1002, 4, 3, 4, 33];

    //let mut instructions = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    //let mut instructions = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    //let mut instructions = vec![
    //    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
    //    1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
    //    1105, 1, 46, 98, 99,
    //];

    let input = 5;

    let mut i = 0;
    while i < instructions.len() {
        let opcode = instructions[i] % 100;
        let p1 = (instructions[i] / 100) % 10;
        let p2 = (instructions[i] / 1000) % 10;
        let p3 = (instructions[i] / 10000) % 10;

        if opcode == 1 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[i + 2 as usize]
            } else {
                instructions[instructions[i + 2] as usize]
            };
            let n3 = instructions[i + 3] as usize;

            instructions[n3] = n1 + n2;

            i += 4;
        } else if opcode == 2 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[i + 2 as usize]
            } else {
                instructions[instructions[i + 2] as usize]
            };
            let n3 = instructions[i + 3] as usize;

            instructions[n3] = n1 * n2;

            i += 4;
        } else if opcode == 3 {
            let n1 = instructions[i + 1 as usize] as usize;
            instructions[n1] = input;
            i += 2;
        } else if opcode == 4 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            println!("{}", n1);
            i += 2;
        } else if opcode == 5 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[i + 2 as usize]
            } else {
                instructions[instructions[i + 2] as usize]
            };

            if n1 != 0 {
                i = n2 as usize;
            } else {
                i += 3;
            }
        } else if opcode == 6 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[i + 2 as usize]
            } else {
                instructions[instructions[i + 2] as usize]
            };

            if n1 == 0 {
                i = n2 as usize;
            } else {
                i += 3;
            }
        } else if opcode == 7 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[i + 2 as usize]
            } else {
                instructions[instructions[i + 2] as usize]
            };
            let n3 = instructions[i + 3] as usize;

            if n1 < n2 {
                instructions[n3] = 1;
            } else {
                instructions[n3] = 0;
            };

            i += 4;
        } else if opcode == 8 {
            let n1 = if p1 == 1 {
                instructions[i + 1 as usize]
            } else {
                instructions[instructions[i + 1] as usize]
            };
            let n2 = if p2 == 1 {
                instructions[i + 2 as usize]
            } else {
                instructions[instructions[i + 2] as usize]
            };
            let n3 = instructions[i + 3] as usize;

            if n1 == n2 {
                instructions[n3] = 1;
            } else {
                instructions[n3] = 0;
            };

            i += 4;
        } else if opcode == 99 {
            break;
        }
    }
}
