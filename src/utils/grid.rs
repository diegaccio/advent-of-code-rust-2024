use super::point::Point;
use std::ops::Index;

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
        let width = matrix.len() as i32;
        let mut height = 0;
        if width > 0 {
            height = matrix[0].len() as i32;
        }
        Grid {
            matrix,
            width,
            height,
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.matrix[index.x as usize][index.y as usize]
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
        let grid = Grid::parse("ab\ncd");
        let point = Point::new(1, 1);
        let foo = grid[point];
        assert_eq!(foo, 'd');

        //grid[point] = 'b';
        //assert_eq!(grid[point], 'b');
        assert_eq!(grid.matrix.len(), 2);
        assert_eq!(grid.matrix[0].len(), 2);
    }
}
