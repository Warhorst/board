use std::cmp::max;
use std::collections::HashMap;

use crate::position::Position;

mod position;

struct Board<T> {
    resizeable: bool,
    dimension: Dimension,
    values: HashMap<Position, T>,
}

impl<T> Board<T> {
    pub fn new(dimension: Dimension) -> Self {
        Board {
            resizeable: false,
            values: HashMap::with_capacity(dimension.field_amount()),
            dimension,
        }
    }

    pub fn new_resizeable(dimension: Dimension) -> Self {
        Board {
            resizeable: true,
            values: HashMap::with_capacity(dimension.field_amount()),
            dimension,
        }
    }

    pub fn clear(&mut self) {
        self.values.clear()
    }

    pub fn get_field(&self, position: Position) -> Option<&T> {
        self.values.get(&position)
    }

    pub fn set_field(&mut self, position: Position, value: T) {
        match (self.resizeable, self.dimension.contains_position(position)) {
            (_, true) => { self.values.insert(position, value); }
            (true, false) => {
                self.dimension.resize(position);
                self.values.insert(position, value);
            }
            _ => return
        }
    }

    pub fn clear_field(&mut self, position: Position) -> Option<T> {
        self.values.remove(&position)
    }

    pub fn iter(&self) -> BoardIter<'_, T> {
        BoardIter {
            dimension_iter: self.dimension.iter(),
            values: &self.values,
        }
    }
}

impl<T> Board<T> where T: ToString {
    /// Print a debug-representation of this board.
    /// This method exists to provide a default print method
    /// without overriding #[std::fmt::Display].
    ///
    /// The parameter 'empty' determines what should be displayed if
    /// some board field is empty.
    pub fn print(&self, empty: &str) {
        let field_strings = self.iter()
            .map(|(_, val_opt)| match val_opt {
                None => empty.to_string(),
                Some(val) => val.to_string()
            })
            .collect::<Vec<_>>();

        let cell_size = Self::calculate_cell_size(empty, &self.dimension, &field_strings);

        let width = self.dimension.width();
        Self::print_width_indexes(width, cell_size.0);
        for i in 0..self.dimension.height() {
            Self::print_row(&i.to_string(), cell_size, &field_strings[(i * width)..(i * width + width)])
        }
        Self::print_width_indexes(width, cell_size.0);
    }

    fn calculate_cell_size(empty: &str, dimension: &Dimension, field_strings: &[String]) -> (usize, usize) {
        let mut cell_size = (0, 0);
        let mut update_size = |string: &str| cell_size = (max(cell_size.0, string.len()), max(cell_size.1, string.lines().count()));

        update_size(empty);
        update_size(&dimension.width().to_string());
        update_size(&dimension.height().to_string());
        field_strings.iter().for_each(|string| update_size(string));

        cell_size
    }

    fn print_width_indexes(width: usize, cell_width: usize) {
        let mut line = String::new();
        line.push_str(Self::whitespace(cell_width + 1).as_str());

        for i in 0..width {
            let index_string = i.to_string();
            line.push_str(&index_string);
            line.push_str(&Self::whitespace(cell_width.checked_sub(index_string.len()).unwrap_or(0) + 1));
        }

        println!("{}", line);
    }

    fn print_row(index_string: &str, cell_size: (usize, usize), strings: &[String]) {
        let mut row_lines = vec![vec![]; cell_size.1];
        for string in strings {
            let mut lines = string.lines();
            (0..cell_size.1)
                .into_iter()
                .for_each(|i| row_lines[i].push(lines.next().unwrap_or("")));
        }

        for (i, field_lines) in row_lines.iter().enumerate() {
            let mut line_string = String::new();

            for line in field_lines {
                line_string.push_str(line);
                line_string.push_str(&Self::whitespace(cell_size.0 + 1 - line.len()));
            }

            line_string = match i {
                0 => format!(
                    "{}{}{}{}",
                    index_string,
                    &Self::whitespace(cell_size.0 + 1 - index_string.len()),
                    line_string,
                    index_string
                ),
                _ => format!(
                    "{}{}",
                    &Self::whitespace(cell_size.0 + 1),
                    line_string
                )
            };

            println!("{}", line_string);
        }
    }

    fn whitespace(length: usize) -> String {
        (0..length).map(|_| String::from(" ")).collect()
    }
}

/// Iterator over all board-positions with their current value.
/// The item-type is (Position, Option<&'a T>). The positions
/// are always in order.
struct BoardIter<'a, T> {
    dimension_iter: DimensionIterator,
    values: &'a HashMap<Position, T>,
}

impl<'a, T> Iterator for BoardIter<'a, T> {
    type Item = (Position, Option<&'a T>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.dimension_iter.next() {
            None => None,
            Some(pos) => Some((pos, self.values.get(&pos)))
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Dimension {
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
    fn iter(&self) -> DimensionIterator {
        DimensionIterator::new(self)
    }
}

struct DimensionIterator {
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
    use crate::{Board, Dimension};
    use crate::position::Position;

    #[test]
    fn get_field_works() {
        let mut board = Board::new(Dimension::new(3, 3));
        let pos = Position::new(1, 1);
        assert_eq!(None, board.get_field(pos));

        board.values.insert(pos, 42);
        assert_eq!(Some(&42), board.get_field(pos))
    }

    /// A not resizeable board should allow to set fields inside its
    /// dimension, but  do nothing if the provided #[Position]
    /// is outside of it.
    #[test]
    fn set_field_not_resizeable_works() {
        let dimension = Dimension::new(3, 3);
        let mut board = Board::new(dimension);
        let pos_in_dimension = Position::new(1, 1);
        let pos_outside_dimension = Position::new(4, 4);

        assert!(dimension.contains_position(pos_in_dimension));
        assert!(!dimension.contains_position(pos_outside_dimension));

        board.set_field(pos_in_dimension, 42);
        assert_eq!(Some(&42), board.values.get(&pos_in_dimension));
        assert_eq!(board.dimension, dimension);

        board.set_field(pos_outside_dimension, 42);
        assert_eq!(None, board.values.get(&pos_outside_dimension));
        assert_eq!(board.dimension, dimension);
    }

    /// A not resizeable board should allow to set fields regardless of its
    /// dimension and resize if necessary.
    #[test]
    fn set_field_resizeable_works() {
        let dimension = Dimension::new(3, 3);
        let mut board = Board::new_resizeable(dimension);
        let pos_in_dimension = Position::new(1, 1);
        let pos_outside_dimension = Position::new(4, 4);

        assert!(dimension.contains_position(pos_in_dimension));
        assert!(!dimension.contains_position(pos_outside_dimension));

        board.set_field(pos_in_dimension, 42);
        assert_eq!(Some(&42), board.values.get(&pos_in_dimension));
        assert_eq!(board.dimension, dimension);

        board.set_field(pos_outside_dimension, 42);
        assert_eq!(Some(&42), board.values.get(&pos_outside_dimension));
        assert_eq!(board.dimension, Dimension::new(5, 5));
    }

    /// If the field at the target position is not empty a set_field call should overwrite its value.
    #[test]
    fn set_field_existing_works() {
        let mut board = Board::<usize>::new(Dimension::new(3, 3));
        let pos = Position::new(0, 0);

        assert_eq!(None, board.get_field(pos));

        board.set_field(pos, 42);
        assert_eq!(Some(&42), board.get_field(pos));

        board.set_field(pos, 43);
        assert_eq!(Some(&43), board.get_field(pos));
    }

    /// A field should be cleared by Board::clear_field.
    /// If the field was empty before, nothing should happen.
    #[test]
    fn clear_field_works() {
        let mut board = Board::<usize>::new(Dimension::new(3, 3));
        let pos = Position::new(0, 0);
        assert_eq!(None, board.get_field(pos));

        board.set_field(pos, 42);
        assert_eq!(Some(&42), board.get_field(pos));

        assert_eq!(Some(42), board.clear_field(pos))
    }

    #[test]
    fn clear_works() {
        let mut board = Board::<usize>::new(Dimension::new(3, 3));
        let pos_a = Position::new(0, 0);
        let pos_b = Position::new(1, 1);

        board.set_field(pos_a, 42);
        assert_eq!(Some(&42), board.get_field(pos_a));
        board.set_field(pos_b, 43);
        assert_eq!(Some(&43), board.get_field(pos_b));

        board.clear();
        assert_eq!(None, board.get_field(pos_a));
        assert_eq!(None, board.get_field(pos_b));
    }

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

    #[test]
    fn print_board_works() {
        let dimension = Dimension::from_origin(Position::default(), 5, 5);
        let mut board: Board<usize> = Board::new(dimension);
        board.set_field(Position::new(2, 2), 42);

        board.print("_")
    }
}
