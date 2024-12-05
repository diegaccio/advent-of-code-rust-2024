use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;
advent_of_code::solution!(4);

fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut result = 0;

    for x in 0..grid.width {
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
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut result = 0;

    let up_left = Point::new(-1, -1);
    let down_right = Point::new(1, 1);
    let up_right = Point::new(1, -1);
    let donw_left = Point::new(-1, 1);

    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            let point = Point::new(x, y);

            if grid[point] == 'A'
                && ((grid[point + up_left] == 'M' && grid[point + down_right] == 'S')
                    || (grid[point + up_left] == 'S' && grid[point + down_right] == 'M'))
                && ((grid[point + up_right] == 'M' && grid[point + donw_left] == 'S')
                    || (grid[point + up_right] == 'S' && grid[point + donw_left] == 'M'))
            {
                result += 1;
            }
        }
    }

    Some(result)
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
        assert_eq!(result, Some(9));
    }
}
