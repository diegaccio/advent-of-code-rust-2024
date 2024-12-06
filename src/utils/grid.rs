use super::point::Point;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Grid<T> {
    pub matrix: Vec<Vec<T>>,
    pub width: i32,
    pub height: i32,
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

    pub fn count(&self, c: char) -> u32 {
        let mut counter = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == c {
                    counter += 1;
                }
            }
        }
        counter
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
        let point = Point::new(0, 2);
        let foo = grid[point];
        assert_eq!(foo, 'e');

        let found_point = grid.find('e');
        assert_eq!(found_point, Some(point));

        //println!("{:?}", grid);

        grid[point] = 'X';
        assert_eq!(grid[point], 'X');

        assert_eq!(grid.count('X'), 1);

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 3);
    }
}
