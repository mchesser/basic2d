use super::Vec2;

/// Rectangle structure
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Rect {
    /// Return the vector to the top-left corner of the rectangle
    #[inline]
    pub fn top_left(&self) -> Vec2<f32> {
        Vec2::new(self.top(), self.left())
    }

    /// Return the vector to the top-left corner of the rectangle
    #[inline]
    pub fn top_right(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width, self.y)
    }

    /// Return the vector to the top-left corner of the rectangle
    #[inline]
    pub fn bottom_left(&self) -> Vec2<f32> {
        Vec2::new(self.x, self.y + self.height)
    }

    /// Return the vector to the top-left corner of the rectangle
    #[inline]
    pub fn bottom_right(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width, self.y + self.height)
    }

    /// Return the vector to the center of the rectangle
    #[inline]
    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width/2.0, self.y + self.height/2.0)
    }

    /// Return the x coordinate of the left side of the rectangle
    #[inline]
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Return the x coordinate of the right side of the rectangle
    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Return the y coordinate of the top of the rectangle
    #[inline]
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Return the y coordinate of the bottom of the rectangle
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Moves the rectangle by a specified vector
    #[inline]
    pub fn move_vec(&mut self, vec: Vec2<f32>) {
        self.x += vec.x;
        self.y += vec.y;
    }

    /// Calculate the intersection area of two rectangles
    #[inline]
    pub fn intersect_area(&self, other: &Rect) -> f32 {
        let x_intersect = self.right().min(other.right()) - self.left().max(other.left());
        let y_intersect = self.bottom().min(other.bottom()) - self.top().max(other.top());

        if x_intersect < 0.0 || y_intersect < 0.0 {
            0.0
        }
        else {
            x_intersect * y_intersect
        }
    }
}
