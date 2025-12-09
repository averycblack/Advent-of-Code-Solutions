#![allow(dead_code)]

use std::ops;

///
/// Coordinates
///

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coordinates(pub i32, pub i32);

impl ops::AddAssign<Coordinates> for Coordinates {
    fn add_assign(&mut self, rhs: Coordinates) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, _rhs: Coordinates) -> Coordinates {
        return Coordinates(self.0 + _rhs.0, self.1 + _rhs.1);
    }
}

impl ops::Sub<Coordinates> for Coordinates {
    type Output = Coordinates;
    fn sub(self, rhs: Coordinates) -> Self::Output {
        Coordinates(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<i32> for Coordinates {
    type Output = Coordinates;
    fn mul(self, rhs: i32) -> Self::Output {
        Coordinates(self.0 * rhs, self.1 * rhs)
    }
}

impl Coordinates {
    pub fn neighbors_cardinal(&self) -> [Coordinates; 4] {
        return [*self + NORTH, *self + EAST, *self + SOUTH, *self + WEST];
    }

    pub fn abs(&self) -> Coordinates {
        Coordinates(self.0.abs(), self.1.abs())
    }

    pub fn min(&self, rhs: &Coordinates) -> Coordinates {
        Coordinates(self.0.min(rhs.0), self.1.min(rhs.1))
    }

    pub fn max(&self, rhs: &Coordinates) -> Coordinates {
        Coordinates(self.0.max(rhs.0), self.1.max(rhs.1))
    }
}

pub const NORTH_IDX: usize = 0;
pub const NORTH: Coordinates = Coordinates(0, -1);
pub const SOUTH: Coordinates = Coordinates(0, 1);
pub const EAST: Coordinates = Coordinates(1, 0);
pub const WEST: Coordinates = Coordinates(-1, 0);

pub const NORTHEAST: Coordinates = Coordinates(1, -1);
pub const SOUTHEAST: Coordinates = Coordinates(1, 1);
pub const SOUTHWEST: Coordinates = Coordinates(-1, 1);
pub const NORTHWEST: Coordinates = Coordinates(-1, -1);

pub const CARDINALS: [Coordinates; 4] = [NORTH, EAST, SOUTH, WEST];

pub const DIRS: [Coordinates; 8] = [
    NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST,
];

///
/// Grid
///

#[derive(Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub max_size: Coordinates,
}

impl<T: Clone + Copy + PartialEq + Eq> Grid<T> {
    pub fn from_string(str: &str, f: fn(char) -> T) -> Grid<T> {
        let grid: Vec<Vec<T>> = str.lines().map(|l| l.chars().map(f).collect()).collect();

        let max_size = Coordinates(grid[0].len() as i32, grid.len() as i32);

        Grid { grid, max_size }
    }

    pub fn get_point(&self, coord: Coordinates) -> Option<T> {
        if !(0..self.max_size.0).contains(&coord.0) || !(0..self.max_size.1).contains(&coord.1) {
            return None;
        }

        Some(self.grid[coord.1 as usize][coord.0 as usize])
    }

    pub fn set_point(&mut self, coord: Coordinates, v: T) {
        self.grid[coord.1 as usize][coord.0 as usize] = v;
    }

    pub fn find_one(&self, f: T) -> Option<Coordinates> {
        for (y, l) in self.grid.iter().enumerate() {
            for (x, v) in l.iter().enumerate() {
                if *v == f {
                    return Some(Coordinates(x as i32, y as i32));
                }
            }
        }

        None
    }

    pub fn get_max_size(&self) -> Coordinates {
        return self.max_size;
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            idx: Coordinates(0, 0),
        }
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    idx: Coordinates,
}

impl<'a, T: Clone + Copy + PartialEq + Eq> Iterator for GridIter<'a, T> {
    type Item = (Coordinates, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.grid.max_size {
            return None;
        }

        let val = self.grid.get_point(self.idx).unwrap();
        let coor = self.idx;
        self.idx.0 += 1;
        if self.idx.0 == self.grid.max_size.0 {
            self.idx.1 += 1;
            self.idx.0 = 0;
            if self.idx.1 == self.grid.max_size.1 {
                self.idx = self.grid.max_size;
            }
        }

        Some((coor, val))
    }
}
