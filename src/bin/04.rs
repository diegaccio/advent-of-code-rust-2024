use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;
use std::collections::HashMap;
advent_of_code::solution!(4);

type Coordinates = (u32, u32);

fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut result = 0;

    /*for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);

            if grid[point] == 'X' {
                for step in DIAGONAL {
                    result += (grid.contains(point + step * 3)
                        && grid[point + step] == 'M'
                        && grid[point + step * 2] == 'A'
                        && grid[point + step * 3] == 'S') as u32;
                }
            }
        }
    }*/

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
