use std::collections::HashMap;

use advent_of_code::utils::{grid::Grid, point::Point};

advent_of_code::solution!(8);

type Input = (Grid<char>, HashMap<char, Vec<Point>>);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut antennas = HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let frequency = grid[point];

            if frequency != '.' {
                antennas
                    .entry(frequency)
                    .or_insert_with(Vec::new)
                    .push(point);
            }
        }
    }

    (grid, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, antennas) = parse(input);
    let mut locations = grid.same_size_with('.');

    for frequency in antennas.values() {
        for &first in frequency {
            for &second in frequency {
                if first != second {
                    let distance = second - first;
                    let antinode = second + distance;

                    if grid.contains(antinode) {
                        //println! {"Point 1: {:?} Point 2: {:?} first {} second {}", first, second, grid[first], grid[second]};
                        locations[antinode] = '#';
                    }
                }
            }
        }
    }
    //println!("{}", locations);

    Some(locations.count('#'))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, antennas) = parse(input);
    let mut locations = grid.same_size_with('.');

    for frequency in antennas.values() {
        for &first in frequency {
            for &second in frequency {
                if first != second {
                    let distance = second - first;
                    let mut antinode = second;

                    while grid.contains(antinode) {
                        //println! {"Point 1: {:?} Point 2: {:?} first {} second {}", first, second, grid[first], grid[second]};
                        locations[antinode] = '#';
                        antinode += distance;
                    }
                }
            }
        }
    }
    //println!("{}", locations);

    Some(locations.count('#'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
