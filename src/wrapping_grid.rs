use grid::Grid;
use std::ops::{Index, IndexMut, Deref, DerefMut};

#[derive(Clone)]
pub struct WrappingGrid<T>(Grid<T>);

impl<T> WrappingGrid<T> {
    #[inline]
    pub fn new(grid: Grid<T>) -> WrappingGrid<T> {
        WrappingGrid(grid)
    }

    #[inline]
    pub fn inner(self) -> Grid<T> {
        self.0
    }
}

impl<T> Deref for WrappingGrid<T> {
    type Target = Grid<T>;

    fn deref(&self) -> &Grid<T> {
        &*self
    }
}

impl<T> DerefMut for WrappingGrid<T> {
    fn deref_mut(&mut self) -> &mut Grid<T> {
        &mut *self
    }
}

impl<T> Index<(i32, i32)> for WrappingGrid<T> {
    type Output = T;

    fn index(&self, (x, y): (i32, i32)) -> &T {
        let wrapped_x = {
            if x >= 0 { x % (self.width() as i32) }
            else { (self.width() as i32) + x % (self.width() as i32) }
        };
        let wrapped_y = {
            if y >= 0 { y % (self.height() as i32) }
            else { (self.height() as i32) + y % (self.height() as i32) }
        };

        &(**self)[(wrapped_x as usize, wrapped_y as usize)]
    }
}

impl<T> IndexMut<(i32, i32)> for WrappingGrid<T> {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut T {
        let wrapped_x = {
            if x >= 0 { x % (self.width() as i32) }
            else { (self.width() as i32) + x % (self.width() as i32) }
        };
        let wrapped_y = {
            if y >= 0 { y % (self.height() as i32) }
            else { (self.height() as i32) + y % (self.height() as i32) }
        };

        &mut (**self)[(wrapped_x as usize, wrapped_y as usize)]
    }
}
