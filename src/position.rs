use std::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    /// Create a Position from two usize values.
    pub fn new_u(x: usize, y: usize) -> Self {
        Self::new(x as isize, y as isize)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        *self + *other
    }
}

impl Add<(isize, isize)> for Position {
    type Output = Position;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Add<(isize, isize)> for &Position {
    type Output = Position;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        *self + rhs
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Position;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        self + (rhs.0 as isize, rhs.1 as isize)
    }
}

impl Add<(usize, usize)> for &Position {
    type Output = Position;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        *self + rhs
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position::new(-self.x, -self.y)
    }
}

impl Neg for &Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        *self - *other
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn add_works() {
        assert_eq!(Position::new(1, 2) + Position::new(2, 3), Position::new(3, 5))
    }

    #[test]
    fn neg_works() {
        assert_eq!(-Position::new(1, 2), Position::new(-1, -2))
    }

    #[test]
    fn sub_works() {
        assert_eq!(Position::new(2, 2) - Position::new(2, 3), Position::new(0, -1))
    }

    #[test]
    fn cmp_works() {
        let zero_zero = Position::new(0, 0);
        let zero_one = Position::new(0, 1);
        let one_zero = Position::new(1, 0);
        let one_one = Position::new(1, 1);

        assert_eq!(true, zero_zero == zero_zero);
        assert_eq!(true, zero_zero < zero_one);
        assert_eq!(true, zero_zero < one_zero);
        assert_eq!(true, zero_zero < one_one);
    }
}