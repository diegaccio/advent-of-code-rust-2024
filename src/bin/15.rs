use advent_of_code::utils::{grid::Grid, point::*};
use std::mem::swap;

advent_of_code::solution!(15);

fn parse_input(input: &str) -> (Grid<char>, Vec<Point>) {
    let (map_section, directions_section) = input.split_once("\n\n").unwrap();
    let map = Grid::parse(map_section);
    let directions = directions_section
        .replace("\n", "")
        .chars()
        .map(|c| {
            if c == '^' {
                UP
            } else if c == 'v' {
                DOWN
            } else if c == '<' {
                LEFT
            } else {
                RIGHT
            }
        })
        .collect::<Vec<Point>>();

    (map, directions)
}

fn narrow(grid: &mut Grid<char>, start: &mut Point, direction: Point) {
    let mut position = *start + direction;
    let mut size = 2;

    // Search for the next wall or open space.
    while grid[position] != '.' && grid[position] != '#' {
        position += direction;
        size += 1;
    }

    // Move items one space in direction.
    if grid[position] == '.' {
        let mut previous = '.';
        let mut position = *start;

        for _ in 0..size {
            swap(&mut previous, &mut grid[position]);
            position += direction;
        }

        // Move robot
        *start += direction;
    }
}

//second part function "stolen" from maneatingape
fn wide(
    grid: &mut Grid<char>,
    start: &mut Point,
    direction: Point,
    todo: &mut Vec<Point>,
    seen: &mut Grid<u32>,
    id: u32,
) {
    // Short circuit if path in front of robot is empty.
    let position = *start;
    let next = position + direction;

    if grid[next] == '.' {
        grid[position] = '.';
        grid[next] = '@';
        *start += direction;
        return;
    }

    // Clear any items from previous push.
    todo.clear();
    todo.push(*start);
    let mut index = 0;

    while index < todo.len() {
        let next = todo[index] + direction;
        index += 1;

        let other = match grid[next] {
            '#' => return, // Return early if there's a wall in the way.
            '[' => RIGHT,
            ']' => LEFT,
            _ => continue, // Open space doesn't add any more items to move.
        };

        // Enqueue the first half of box directly above us.
        let first = next;
        if seen[first] != id {
            seen[first] = id;
            todo.push(first);
        }

        // Enqueue the other half of the box directly above us.
        let second = next + other;
        if seen[second] != id {
            seen[second] = id;
            todo.push(second);
        }
    }

    // Move boxes in reverse order.
    for &point in todo.iter().rev() {
        grid[point + direction] = grid[point];
        grid[point] = '.';
    }

    // Move robot
    *start += direction;
}

fn calc_gps_coord(map: &Grid<char>, char_to_count: char) -> u32 {
    let mut gps_coordinates = 0;
    //skip the frame
    for x in 1..map.width - 1 {
        for y in 1..map.height - 1 {
            let point = Point::new(x, y);

            if map[point] == char_to_count {
                gps_coordinates += 100 * point.y + point.x
            }
        }
    }

    gps_coordinates.try_into().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, directions) = parse_input(input);

    //println!("{}\n", map);

    let mut current_point = map.find('@').unwrap();

    for direction in directions {
        narrow(&mut map, &mut current_point, direction);
    }

    Some(calc_gps_coord(&map, 'O'))
}

fn expand_map(map: Grid<char>) -> Grid<char> {
    let mut expanded_map: Grid<char> = Grid::<char>::new(map.width * 2, map.height, '#');

    //skip the horizontal frame
    for x in 0..map.width {
        for y in 1..map.height - 1 {
            let src = Point::new(x, y);

            let dst = Point::new(x * 2, y);
            //let second_point = dst + RIGHT;

            expanded_map[dst] = map[src];
            if map[src] == '.' || map[src] == '#' {
                expanded_map[dst + RIGHT] = map[src];
            } else if map[src] == '@' {
                expanded_map[dst + RIGHT] = '.';
            } else if map[src] == 'O' {
                expanded_map[dst] = '[';
                expanded_map[dst + RIGHT] = ']';
            }
        }
    }

    expanded_map
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, directions) = parse_input(input);

    let mut map = expand_map(map);

    //println!("{}\n", map);

    let mut current_point = map.find('@').unwrap();

    // Reuse to minimize allocations.
    let mut todo = Vec::new();
    let mut seen = map.same_size_with(u32::MAX);

    // Use index as a unique id for each move.
    for (id, direction) in directions.into_iter().enumerate() {
        if direction == RIGHT || direction == LEFT {
            narrow(&mut map, &mut current_point, direction);
        } else {
            wide(
                &mut map,
                &mut current_point,
                direction,
                &mut todo,
                &mut seen,
                id as u32,
            );
        }
    }

    Some(calc_gps_coord(&map, '['))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
