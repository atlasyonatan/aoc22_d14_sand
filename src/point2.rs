use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Sub> Sub for Point2<T> {
    type Output = Point2<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Add> Add for Point2<T> {
    type Output = Point2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
