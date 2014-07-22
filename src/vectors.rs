use std::fmt;
use std::num::{zero, one, Float, FloatMath};

pub type Vec2<T> = [T, ..2];

impl<T: Clone + Num> BasicVector2<T> for Vec2<T> {
    fn as_raw(&self) -> RawVector2<T> {
        RawVector2::new(self[0].clone(), self[1].clone())
    }

    fn from_raw(raw: RawVector2<T>) -> Vec2<T> {
        [raw.x, raw.y]
    }
}

pub trait BasicVector2<T: Clone + Num> {
    fn as_raw(&self) -> RawVector2<T>;

    fn from_raw(raw: RawVector2<T>) -> Self;

    /// Create a new vector of length 0
    fn zero() -> Self {
        BasicVector2::from_raw(RawVector2::<T>::zero())
    }

    /// Create the unit vector in the x direction
    fn unit_x() -> Self {
        BasicVector2::from_raw(RawVector2::<T>::unit_x())
    }

    /// Create the unit vector in the y direction
    fn unit_y() -> Self {
        BasicVector2::from_raw(RawVector2::<T>::unit_y())
    }

    /// Calculate the dot product between this and another vector
    ///
    /// # Arguments
    /// `other` - the other vector
    fn dot(&self, other: &Self) -> T {
        let a = self.as_raw();
        let b = other.as_raw();
        a.dot(&b)
    }
}

pub struct RawVector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> RawVector2<T> {
    pub fn new(x: T, y: T) -> RawVector2<T> {
        RawVector2 {
            x: x,
            y: y,
        }
    }
}

impl<T: Clone + Num> RawVector2<T> {
    /// Create a new vector of length 0
    pub fn zero() -> RawVector2<T> {
        RawVector2::new(zero(), zero())
    }

    /// Create the unit vector in the x direction
    pub fn unit_x() -> RawVector2<T> {
        RawVector2::new(one(), zero())
    }

    /// Create the unit vector in the y direction
    pub fn unit_y() -> RawVector2<T> {
        RawVector2::new(zero(), one())
    }

    /// Calculate the dot product between this and another vector
    ///
    /// # Arguments
    /// `other` - the other vector
    pub fn dot(&self, other: &RawVector2<T>) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<T: Clone + Num> Add<RawVector2<T>, RawVector2<T>> for RawVector2<T> {
    /// Adds two vectors together
    fn add(&self, rhs: &RawVector2<T>) -> RawVector2<T> {
        RawVector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Clone + Num> Sub<RawVector2<T>, RawVector2<T>> for RawVector2<T> {
    /// Subtracts one vector from another
    fn sub(&self, rhs: &RawVector2<T>) -> RawVector2<T> {
        RawVector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Float + FloatMath> RawVector2<T> {
    /// Create a new vector from polar coordinates
    ///
    /// # Arguments
    /// `angle` - the angle of the vector
    /// `mag` - the magnitude of the vector
    pub fn from_polar(angle: T, mag: T) -> RawVector2<T> {
        let (sin_a, cos_a) = angle.sin_cos();
        RawVector2::new(mag * cos_a, mag * sin_a)
    }

    /// Calculates the length squared of the vector. Avoids taking a square root.
    pub fn length_sqr(&self) -> T {
        self.dot(self)
    }

    /// Calculates the lenght of the vector
    pub fn length(&self) -> T {
        self.length_sqr().sqrt()
    }

    /// Normalises the vector
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
    }

    /// Creates a unit vector in the direction of the vector
    pub fn unit(&self) -> RawVector2<T> {
        let len = self.length();
        RawVector2::new(self.x / len, self.y / len)
    }

    /// Rotates a vector by a specified angle
    ///
    /// # Arguments
    /// `angle` - the angle to rotate by
    pub fn rotate(&mut self, angle: T) {
        let (cos_a, sin_a) = angle.sin_cos();
        let (old_x, old_y) = (self.x.clone(), self.y.clone());
        self.x = old_x * cos_a - old_y * sin_a;
        self.y = old_x * sin_a + old_y * cos_a;
    }

    /// Gets the angle of the vector
    pub fn angle(&self) -> T {
        self.x.atan2(self.y)
    }
}

impl<T: Mul<T, T>> RawVector2<T> {
    /// Creates a new vector equal to the vector scaled by a constant
    ///
    /// # Arguments
    /// `scalar` - the scalar to use
    pub fn scale(&self, scalar: T) -> RawVector2<T> {
        RawVector2::new(self.x * scalar, self.y * scalar)
    }
}

impl<T: fmt::Show> fmt::Show for RawVector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
