use std::slice;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T> {
	width: usize,
	height: usize,
	data: Vec<T>,
}

impl<T> Grid<T> {
	/// Construct a new grid filled with a specified element
	#[inline]
	pub fn from_elem(width: usize, height: usize, elem: T) -> Grid<T> where T: Clone {
		Grid {
			width: width,
			height: height,
			data: vec![elem; width * height],
		}
	}

	/// Construct a new grid filled with values specified by a generating function
	#[inline]
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Grid<T>
        where F: FnMut(usize, usize) -> T
    {
        Grid {
            width: width,
            height: height,
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

    /// Returns an iterator over references to the elements of the grid in
    /// the order: left-right, up-down
	#[inline]
    pub fn iter(&self) -> slice::Iter<T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the elements of the grid in
    /// the order: left-right, up-down
	#[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.data.iter_mut()
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
