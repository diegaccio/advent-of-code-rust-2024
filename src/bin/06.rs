advent_of_code::solution!(6);
use std::collections::HashMap;

use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input)
}

const MOVES: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

fn part_one_walk(mut grid: Grid<char>) -> Grid<char> {
    let mut current_point = grid.find('^').unwrap();
    grid[current_point] = 'X';
    let mut move_index = 0;

    while grid.contains(current_point + MOVES[move_index]) {
        while grid[current_point + MOVES[move_index]] == '#' {
            move_index += 1;
            if move_index == MOVES.len() {
                move_index = 0;
            }
        }

        current_point += MOVES[move_index];
        grid[current_point] = 'X';
    }

    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    grid = part_one_walk(grid);

    Some(grid.count('X'))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    //before part one walk
    let starting_point = grid.find('^').unwrap();

    //get all the Xs of the original path
    //for each of them I'll try to insert an obstable
    grid = part_one_walk(grid);

    //reset the starting point
    grid[starting_point] = '^';

    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);
            let original_char = grid[point];
            if grid[point] == 'X' {
                //place obstacle
                grid[point] = '#';

                let mut current_point = starting_point;

                let mut move_index = 0;

                let mut map: HashMap<Point, Vec<Point>> = HashMap::new();

                let mut loop_detected = false;

                while grid.contains(current_point + MOVES[move_index]) && !loop_detected {
                    while grid[current_point + MOVES[move_index]] == '#' {
                        move_index += 1;
                        if move_index == MOVES.len() {
                            move_index = 0;
                        }
                    }

                    current_point += MOVES[move_index];
                    // check loop
                    // if we arrive at the same point going the same direction then there is a loop
                    let directions = map.entry(current_point).or_default(); //or_insert_with(Vec::new);
                    if directions.contains(&MOVES[move_index]) {
                        loop_detected = true;
                    }
                    directions.push(MOVES[move_index]);
                }
                if loop_detected {
                    grid[point] = 'O';
                } else {
                    grid[point] = original_char;
                }
            }
        }
    }

    Some(grid.count('O'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
