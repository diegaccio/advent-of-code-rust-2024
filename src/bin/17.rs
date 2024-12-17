advent_of_code::solution!(17);

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

pub fn part_one(input: &str) -> Option<String> {
    //println!("{}", input);
    let (registers, instructions) = input.split_once("\n\n").unwrap();

    let mut registers: Vec<u32> = registers
        .lines()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .parse()
                .expect("Cannot parse integer")
        })
        .collect();

    //println!("{:?}", registers);

    let instructions: Vec<u32> = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse().expect("Cannot parse integer"))
        .collect();

    let mut instruction_pointer = 0;

    let mut output_vec = vec![];

    //println!("{:?}", instructions);

    while instruction_pointer < instructions.len() {
        let combo = |index| match instructions[index] {
            0..4 => instructions[index],
            4 => registers[A],
            5 => registers[B],
            6 => registers[C],
            _ => unreachable!(),
        };

        /*println!(
            "Instruction: {} Operand: {}",
            instructions[instruction_pointer],
            instructions[instruction_pointer + 1]
        );*/

        match instructions[instruction_pointer] {
            //The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
            //The denominator is found by raising 2 to the power of the instruction's combo operand.
            //(So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
            //The result of the division operation is truncated to an integer and then written to the A register.
            //shift right
            0 => registers[A] >>= combo(instruction_pointer + 1),
            //The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
            //then stores the result in register B.
            1 => registers[B] ^= instructions[instruction_pointer + 1],
            //The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
            //(thereby keeping only its lowest 3 bits), then writes that value to the B register.
            2 => registers[B] = combo(instruction_pointer + 1) % 8,
            //The jnz instruction (opcode 3) does nothing if the A register is 0.
            //However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
            //if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            3 => {
                if registers[A] != 0 {
                    instruction_pointer = instructions[instruction_pointer + 1] as usize;
                    continue;
                }
            }
            //The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
            //then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
            4 => registers[B] ^= registers[C],
            //The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
            //(If a program outputs multiple values, they are separated by commas.)
            5 => {
                let out = combo(instruction_pointer + 1) % 8;
                //instruction_pointer += 2;
                output_vec.push(out);
                //println!("{:?}", output_vec);
            }
            //The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
            //(The numerator is still read from the A register.)
            6 => registers[B] = registers[A] >> combo(instruction_pointer + 1),
            //The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
            //(The numerator is still read from the A register.)
            7 => registers[C] = registers[A] >> combo(instruction_pointer + 1),
            _ => unreachable!(),
        }

        //println!("{:?}", registers);

        instruction_pointer += 2;
    }

    //println!("{:?}", registers);
    //println!("{:?}", output_vec);

    let output_string = output_vec
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Some(output_string)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("0,1,2".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some("4,2,5,6,7,7,7,7,3,1,0".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
