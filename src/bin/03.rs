advent_of_code::solution!(3);

//custom parser from maneatingape
//10 time faster than my solution
/*
pub fn parse(input: &str) -> (u32, u32) {
    let memory = input.as_bytes();
    let mut index = 0;
    let mut enabled = true;
    let mut part_one = 0;
    let mut part_two = 0;

    while index < memory.len() {
        // Skip junk characters
        if memory[index] != b'm' && memory[index] != b'd' {
            index += 1;
            continue;
        }

        // Check possible prefixes
        if memory[index..].starts_with(b"mul(") {
            index += 4;
        } else if memory[index..].starts_with(b"do()") {
            index += 4;
            enabled = true;
            continue;
        } else if memory[index..].starts_with(b"don't()") {
            index += 7;
            enabled = false;
            continue;
        } else {
            index += 1;
            continue;
        }

        // First number
        let mut first = 0;

        while memory[index].is_ascii_digit() {
            first = 10 * first + (memory[index] - b'0') as u32;
            index += 1;
        }

        // First delimiter
        if memory[index] != b',' {
            continue;
        }
        index += 1;

        // Second number
        let mut second = 0;

        while memory[index].is_ascii_digit() {
            second = 10 * second + (memory[index] - b'0') as u32;
            index += 1;
        }

        // Second delimiter
        if memory[index] != b')' {
            continue;
        }
        index += 1;

        // Multiply
        //println!("[{}, {}]", first, second);
        let product = first * second;
        part_one += product;
        if enabled {
            part_two += product;
        }
    }

    (part_one, part_two)
}
    */

fn chek_number(to_parse: &str) -> Option<u32> {
    if (to_parse.len() > 3) || !to_parse.chars().all(|c| c.is_numeric()) {
        return None;
    }
    let res = to_parse.parse::<u32>();
    match res {
        Ok(number) => {
            //println!("OK: {number}");
            Some(number)
        }
        Err(_) => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("mul(")
            .filter_map(|s| {
                if !s.contains(")") && !s.chars().next().unwrap().is_numeric() {
                    return None;
                }

                let comma_separated_numbers: Vec<&str> =
                    s.split(')').next().unwrap().split(",").collect();

                if comma_separated_numbers.len() != 2 {
                    return None;
                }

                //println!("{:?}", comma_separated_numbers);

                let numbers = comma_separated_numbers
                    .iter()
                    .filter_map(|s| chek_number(s))
                    .collect::<Vec<u32>>();

                if numbers.len() != 2 {
                    return None;
                }
                //println!("{:?}", numbers);
                Some(numbers.iter().product::<u32>())

                //println!("{:?}", comma_separated_numbers);
            })
            .sum(),
    )

    /*let (first, _) = parse(input);
    Some(first)*/
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = 0;
    let input_for_part_one = input
        .split("don't()")
        .map(|s| {
            counter += 1;
            let after: String;
            //println!("\nBefore : {s}\n");
            if counter == 1 {
                after = s.to_string()
            } else if s.contains("do()") {
                after = s.split("do()").skip(1).collect::<Vec<&str>>().join("")
            } else {
                after = "".to_string()
            }
            //println!("\nAfter: {after}\n\n");
            after
        })
        .collect::<Vec<String>>()
        .join("");

    //println!("{input_for_part_one}");

    part_one(&input_for_part_one)

    /*let (_, second) = parse(input);
    Some(second)*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
