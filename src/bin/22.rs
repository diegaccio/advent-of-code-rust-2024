advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let secrets: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let iterations = 2000;
    let mut sum = 0;

    for &initial_secret in &secrets {
        let mut secret = initial_secret;
        for _ in 0..iterations {
            secret = next_secret(secret);
        }
        sum += secret;
        /*println!(
            "Buyer starting with {}: 2000th secret is {}",
            initial_secret, secret
        );*/
    }

    /*println!("Sum of 2000th secrets: {}", sum);*/
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u16> {
    let secrets: Vec<usize> = input.lines().map(|s| s.parse().unwrap()).collect();

    let mut part_two = vec![0; 130321];
    let mut seen = vec![u16::MAX; 130321];

    for (id, number) in secrets.iter().enumerate() {
        let id = id as u16;

        let zeroth = *number;
        let first = hash(zeroth);
        let second = hash(first);
        let third = hash(second);

        let mut a;
        let mut b = to_index(zeroth, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);

        let mut number = third;
        let mut previous = third % 10;

        for _ in 3..2000 {
            number = hash(number);
            let price = number % 10;

            (a, b, c, d) = (b, c, d, 9 + price - previous);
            let index = 6859 * a + 361 * b + 19 * c + d;

            if seen[index] != id {
                part_two[index] += price as u16;
                seen[index] = id;
            }

            previous = price;
        }
    }

    Some(*part_two.iter().max().unwrap())

    //    None
}

fn hash(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

/// Convert -9..9 to 0..18.
fn to_index(previous: usize, current: usize) -> usize {
    9 + current % 10 - previous % 10
}

fn next_secret(secret: u64) -> u64 {
    let mut result = secret;

    // Step 1: Multiply by 64, XOR, and prune
    result ^= result.wrapping_mul(64);
    result %= 16777216;

    // Step 2: Divide by 32, XOR, and prune
    result ^= result / 32;
    result %= 16777216;

    // Step 3: Multiply by 2048, XOR, and prune
    result ^= result.wrapping_mul(2048);
    result %= 16777216;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
