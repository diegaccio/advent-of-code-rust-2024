advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let vec: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let v: Vec<usize> = line
                .split_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect();
            (v[0], v[1])
        })
        .collect();

    vec.into_iter().unzip()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut x, mut y) = parse_input(input);

    //Sorts the slice without preserving the initial order of equal elements.
    //faster than sort: doesn't allocate
    x.sort_unstable();
    y.sort_unstable();

    let mut count: usize = 0;
    for i in 0..x.len() {
        if y[i] >= x[i] {
            count += y[i] - x[i];
        } else {
            count += x[i] - y[i];
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (x, y) = parse_input(input);

    Some(
        x.into_iter()
            .map(|v| y.iter().filter(|&n| *n == v).count() * v)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
