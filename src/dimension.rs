use crate::position::Position;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dimension {
    pub origin: Position,
    pub max: Position,
}

impl Dimension {
    /// Create a dimension with the default #[Position] (0, 0) as origin.
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_origin(Position::default(), width, height)
    }

    /// Create a dimension with a custom origin.
    pub fn from_origin(origin: Position, width: usize, height: usize) -> Self {
        if width == 0 || height == 0 {
            panic!("Cannot create dimension with zero width or height!")
        }

        Dimension {
            origin,
            max: origin + (width - 1, height - 1),
        }
    }

    pub fn width(&self) -> usize {
        (self.max.x - self.origin.x + 1) as usize
    }

    pub fn height(&self) -> usize {
        (self.max.y - self.origin.y + 1) as usize
    }

    /// Return how many fields a board can hold with this dimension.
    pub fn field_amount(&self) -> usize {
        self.width() * self.height()
    }

    /// Return if a given position is covered by this dimension.
    pub fn contains_position(&self, position: Position) -> bool {
        self.origin <= position && position <= self.max
    }

    /// Resize this dimension if the given #[Position] exceeds its bonds.
    pub fn resize(&mut self, position: Position) {
        if position < self.origin {
            self.origin = position
        }

        if position > self.max {
            self.max = position
        }
    }

    /// Return an iterator over all possible #[Position]s of this dimension.
    ///
    /// The returned positions are in order, starting with self.origin and ending with
    /// self.origin + (self.width, self.height).
    pub fn iter(&self) -> DimensionIterator {
        DimensionIterator::new(self)
    }
}

pub struct DimensionIterator {
    origin: Position,
    current_position: Option<Position>,
    max_position: Position,
}

impl DimensionIterator {
    pub fn new(dimension: &Dimension) -> Self {
        DimensionIterator {
            origin: dimension.origin,
            current_position: None,
            max_position: dimension.max,
        }
    }
}

impl Iterator for DimensionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let x_max = self.max_position.x;
        let y_max = self.max_position.y;

        let next = match self.current_position {
            None => self.origin,
            Some(pos) if pos == self.max_position => return None,
            Some(Position { x, y }) if y == y_max && x < x_max => Position::new(x + 1, 0),
            Some(Position { x, y }) => Position::new(x, y + 1)
        };

        self.current_position = Some(next);
        Some(next)
    }
}
#[cfg(test)]
mod tests {
    use crate::dimension::Dimension;
    use crate::position::Position;

    #[test]
    fn dimension_iterator_works() {
        let dimension = Dimension::from_origin(Position::default(), 3, 3);
        let positions_in_dimension = dimension.iter().collect::<Vec<_>>();

        assert_eq!(vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ], positions_in_dimension)
    }
}