use std::ops::{Index, IndexMut};
use std::slice;

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    /// Construct a new grid filled with a specified element
    #[inline]
    pub fn from_elem(width: usize, height: usize, elem: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid { width, height, data: vec![elem; width * height] }
    }

    /// Construct a new grid filled with values specified by a generating function
    #[inline]
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Grid<T>
    where
        F: FnMut(usize, usize) -> T,
    {
        Grid {
            width,
            height,
            data: (0..(width * height)).map(|i| f(i % width, i / width)).collect(),
        }
    }

    /// Returns the height of the grid
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the width of the grid
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns an iterator over references to the elements of the grid.
    #[inline]
    pub fn iter(&self) -> slice::Iter<T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the elements of the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.data.iter_mut()
    }

    /// Return an iterator over the coordinates of the grid.
    #[inline]
    pub fn coordinates(&self) -> Coordinates {
        Coordinates { width: self.width, height: self.height, x: 0, y: 0 }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        assert!(x < self.width && y < self.height);
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        assert!(x < self.width && y < self.height);
        &mut self.data[x + y * self.width]
    }
}

pub struct Coordinates {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Iterator for Coordinates {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.x < self.width {
            self.x += 1;
            Some((self.x - 1, self.y))
        }
        else {
            self.x = 0;
            self.y += 1;

            if self.y < self.height { self.next() } else { None }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.width * self.height - self.x - self.width * self.y;
        (size, Some(size))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construction_test() {
        let grid = Grid::from_elem(2, 3, 75_u32);
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.iter().cloned().collect::<Vec<_>>(), vec![75, 75, 75, 75, 75, 75]);
    }

    #[test]
    fn index_test() {
        let mut grid = Grid::from_elem(2, 3, 75_u32);

        grid[(0, 0)] = 0;
        grid[(1, 0)] = 10;
        grid[(0, 1)] = 1;
        grid[(1, 1)] = 11;
        grid[(0, 2)] = 2;
        grid[(1, 2)] = 12;

        assert_eq!(grid.iter().cloned().collect::<Vec<_>>(), vec![0, 10, 1, 11, 2, 12]);
    }

    #[test]
    fn coord_iter_test() {
        let grid = Grid::from_elem(3, 3, 0_u32);
        assert_eq!(grid.coordinates().collect::<Vec<_>>(), vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2)
        ]);
    }
}
