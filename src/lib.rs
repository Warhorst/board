use std::cmp::max;
use std::collections::HashMap;

use crate::position::Position;
use crate::dimension::{Dimension, DimensionIterator};

mod position;
mod dimension;

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

#[cfg(test)]
mod tests {
    use crate::{Board};
    use crate::position::Position;
    use crate::dimension::Dimension;

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
    fn print_board_works() {
        let dimension = Dimension::from_origin(Position::default(), 5, 5);
        let mut board: Board<usize> = Board::new(dimension);
        board.set_field(Position::new(2, 2), 42);

        board.print("_")
    }
}
