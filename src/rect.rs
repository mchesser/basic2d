//! Structures and functions for managing rectangles
use vectors::Vec2;

/// Get the minimum of two values
fn min<T: PartialOrd + Clone>(n1: &T, n2: &T) -> T {
    if n1 < n2 {
        n1.clone()
    }
    else {
        n2.clone()
    }
}

/// Get the maximum of two values
fn max<T: PartialOrd + Clone>(n1: &T, n2: &T) -> T {
    if n1 > n2 {
        n1.clone()
    }
    else {
        n2.clone()
    }
}

/// Rectangle structure
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T
}

impl<T: Num + PartialOrd + Clone> Rect<T> {
    /// Gets the top left coordinate of the rectangle
    pub fn top_left(&self) -> Vec2<T> {
        [self.left(), self.top()]
    }

    /// Gets the top right coordinate of the rectangle
    pub fn top_right(&self) -> Vec2<T> {
        [self.right(), self.top()]
    }

    /// Gets the bottom left coordinate of the rectangle
    pub fn bottom_left(&self) -> Vec2<T> {
        [self.left(), self.bottom()]
    }

    /// Gets the bottom right coordinate of the rectangle
    pub fn bottom_right(&self) -> Vec2<T> {
        [self.right(), self.bottom()]
    }

    /// Gets the x value of the left of the rectangle
    pub fn left(&self) -> T {
        self.x.clone()
    }

    /// Gets the x value of the right of the rectangle
    pub fn right(&self) -> T {
        self.x.clone() + self.width.clone()
    }

    /// Gets the y value of the top of the rectangle
    pub fn top(&self) -> T {
        self.y.clone()
    }

    /// Gets the y value of the bottom of the rectangle
    pub fn bottom(&self) -> T {
        self.y.clone() + self.height.clone()
    }

    pub fn area(&self) -> T {
        self.width.clone() * self.height.clone()
    }

    /// Calculate the intersection of two rectangles, returning None if the two rectangles do not
    /// intersect.
    pub fn intersect(&self, other: &Rect<T>) -> Option<Rect<T>> {
        if self.right() < other.left() || self.left() > other.right() ||
                self.bottom() < other.top() || self.top() > other.bottom() {
            None
        }
        else {
            let x = max(&self.x, &other.x);
            let y = max(&self.y, &other.y);

            Some(Rect {
                x: x.clone(),
                y: y.clone(),
                width: min(&self.right(), &other.right()) - x,
                height: min(&self.bottom(), &other.bottom()) - y,
            })
        }
    }
}
