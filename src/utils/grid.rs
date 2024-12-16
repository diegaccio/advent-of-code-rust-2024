use super::point::Point;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub matrix: Vec<Vec<T>>,
    pub width: i32,
    pub height: i32,
}

impl Grid<u32> {
    pub fn u32_parse(input: &str) -> Self {
        let matrix = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let height = matrix.len() as i32;
        let mut width = 0;
        if height > 0 {
            width = matrix[0].len() as i32;
        }
        Grid {
            matrix,
            width,
            height,
        }
    }

    pub fn count(&self, value: u32) -> u32 {
        self.matrix
            .iter()
            .map(|v| v.iter().filter(|x| **x == value).count())
            .sum::<usize>() as u32
    }
}

impl Grid<char> {
    pub fn parse(input: &str) -> Self {
        let matrix = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let height = matrix.len() as i32;
        let mut width = 0;
        if height > 0 {
            width = matrix[0].len() as i32;
        }
        Grid {
            matrix,
            width,
            height,
        }
    }

    pub fn new(width: i32, height: i32, value: char) -> Grid<char> {
        Grid {
            width,
            height,
            matrix: vec![vec![value; width as usize]; height as usize],
        }
    }

    pub fn find(&self, c: char) -> Option<Point> {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == c {
                    return Some(Point::new(x as i32, y as i32));
                }
            }
        }

        None
    }

    pub fn line_contains_string(&self, to_find: &str) -> bool {
        for y in 0..self.matrix.len() {
            if self.matrix[y].iter().collect::<String>().contains(to_find) {
                return true;
            }
        }
        false
    }

    pub fn count(&self, c: char) -> u32 {
        self.matrix
            .iter()
            .map(|v| v.iter().filter(|x| **x == c).count())
            .sum::<usize>() as u32
    }
}

impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.matrix.len() {
            writeln!(f, "{}", self.matrix[y].iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.matrix[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.matrix[index.y as usize][index.x as usize]
    }
}

impl<T> Grid<T> {
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            matrix: vec![vec![value; self.width as usize]; self.height as usize],
        }
    }

    /*pub fn new<U: Copy>(width: i32, height: i32, value: U) -> Grid<U> {
        Grid {
            width,
            height,
            matrix: vec![vec![value; width as usize]; height as usize],
        }
    }*/

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::super::point::Point;
    use super::*;
    //use advent_of_code::utils::grid::Grid;
    //use advent_of_code::utils::point::Point;

    #[test]
    fn test_grid() {
        let mut grid = Grid::parse("ab\ncd\nef");
        let mut point = Point::new(0, 2);
        let foo = grid[point];
        assert_eq!(foo, 'e');

        let found_point = grid.find('e');
        assert_eq!(found_point, Some(point));

        //println!("{:?}", grid);

        grid[point] = 'X';
        assert_eq!(grid[point], 'X');

        point = Point::new(1, 2);
        grid[point] = 'X';

        println!("{}", grid);

        assert_eq!(grid.count('X'), 2);

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 3);
    }

    #[test]
    fn test_u32_grid() {
        let grid = Grid::u32_parse("12\n23\n56");
        let point = Point::new(0, 2);
        let foo = grid[point];
        assert_eq!(foo, 5);

        /*let found_point = grid.find('e');
        assert_eq!(found_point, Some(point));

        //println!("{:?}", grid);

        grid[point] = 'X';
        assert_eq!(grid[point], 'X');

        point = Point::new(1, 2);
        grid[point] = 'X';

        println!("{}", grid);*/

        assert_eq!(grid.count(3), 1);
        assert_eq!(grid.count(4), 0);

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 3);

        let new_grid = grid.clone();
        assert_eq!(new_grid.count(3), 1);
    }
}
