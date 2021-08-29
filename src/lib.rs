use std::collections::HashMap;
use crate::position::Position;

mod position;
mod board;

type Values<T> = HashMap<Position, T>;

/// A field is a 2D board used used in games.
/// It has no fixed size, a specific origin and only saves fields which are not empty.
pub struct Field<T> {
    width: usize,
    height: usize,
    origin: Position,
    values: Values<T>,
}

impl <T> Field<T> where T: PartialEq {
    pub fn empty(width: usize, height: usize) -> Self {
        Field { width, height, origin: Position::new(0, 0), values: HashMap::new() }
    }

    pub fn iter(&self) -> Iter<T> {
        todo!()
    }
}

pub struct Iter<'a, T> {
    index: usize,
    values: Vec<(Position, &'a T)>
}

impl<'a, T> Iter<'a, T> {
    pub fn new(field: &Field<T>) {
        todo!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.index == self.values.len() {
            true => None,
            false => {
                let item = self.values.get(self.index);
                self.index += 1;
                item
            }
        };

        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
