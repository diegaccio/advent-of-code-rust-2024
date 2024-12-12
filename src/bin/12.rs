use std::collections::HashSet;

use advent_of_code::utils::{grid::Grid, point::*};

advent_of_code::solution!(12);

fn count_slot(coordinates: Point, visited: &mut HashSet<Point>, field: &Grid<char>) -> (u64, u64) {
    if visited.contains(&coordinates) {
        return (0, 0);
    }

    visited.insert(coordinates);

    let mut current_perimeter = 4;
    let mut current_area = 1;

    for next in ORTHOGONAL.map(|o| coordinates + o) {
        if field.contains(next) && field[next] == field[coordinates]
        //&& (allow_already_visited || (visited[next] != index as u32))
        {
            //that side doesn't count for the perimeter
            current_perimeter -= 1;
            let (others_perimeter, others_area) = count_slot(next, visited, field);
            current_perimeter += others_perimeter;
            current_area += others_area;
        }
    }

    (current_perimeter, current_area)
}

pub fn part_one(input: &str) -> Option<u64> {
    let field = Grid::parse(input);

    //141 * 141 points = 19_881
    let visited: &mut HashSet<Point> = &mut HashSet::with_capacity(20_000);

    let mut total_price = 0;
    //let mut total_area = 0;

    for y in 0..field.height {
        for x in 0..field.width {
            let point = Point::new(x, y);
            let (current_perimeter, current_area) = count_slot(point, visited, &field);
            total_price += current_perimeter * current_area;
        }
    }

    Some(total_price)
}

fn count_slot_2(
    coordinates: Point,
    visited: &mut HashSet<Point>,
    field: &Grid<char>,
) -> (u64, u64) {
    if visited.contains(&coordinates) {
        return (0, 0);
    }

    visited.insert(coordinates);

    let mut current_edges = 0;
    let mut edges: HashSet<Point> = HashSet::from_iter(STRICTLY_DIAGONAL);

    let mut current_area = 1;

    println!(
        "Coordinates {:?} Value {} current_perimeter {}",
        coordinates, field[coordinates], current_edges
    );

    for next in ORTHOGONAL.map(|o| coordinates + o) {
        if field.contains(next) && field[next] == field[coordinates] {
            let (others_edges, others_area) = count_slot_2(next, visited, field);
            current_edges += others_edges;
            current_area += others_area;

            if next == UP || next == RIGHT {
                edges.remove(&UP_RIGHT);
            }

            if next == UP || next == LEFT {
                edges.remove(&UP_LEFT);
            }
            if next == DOWN || next == RIGHT {
                edges.remove(&DOWN_RIGHT);
            }
            if next == DOWN || next == LEFT {
                edges.remove(&DOWN_LEFT);
            }
        }
    }

    current_edges += edges.len() as u64;
    (current_edges, current_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let field = Grid::parse(input);

    //141 * 141 points = 19_881
    let visited: &mut HashSet<Point> = &mut HashSet::with_capacity(20_000);

    let mut total_price = 0;
    //let mut total_area = 0;

    for y in 0..field.height {
        for x in 0..field.width {
            let point = Point::new(x, y);
            let (current_perimeter, current_area) = count_slot_2(point, visited, &field);
            total_price += current_perimeter * current_area;
        }
    }

    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(436));
    }
}
