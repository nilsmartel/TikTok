#[cfg(test)]
mod tests {
    #[test]
    fn rotate_right() {
        use super::rotate_id_right as right;
        for i in 0..9 {
            assert_eq!(
                i,
                right(right(right(right(i)))),
                "testing rotation right of field #{}",
                i
            );
        }
    }

    #[test]
    fn rotate_left() {
        use super::rotate_id_left as left;
        for i in 0..9 {
            assert_eq!(
                i,
                left(left(left(left(i)))),
                "testing rotation left of field #{}",
                i
            );
        }
    }

    #[test]
    fn rotate_180() {
        use super::rotate_id_180 as rot;
        for i in 0..9 {
            assert_eq!(i, rot(rot(i)), "testing 180° rotation of field {}", i);
        }
    }

    #[test]
    fn state_encoding_1() {
        use super::{Entry, GameState};

        let code = 2;

        let state = GameState {
            fields: [
                Entry::Human,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
            ],
        };

        assert_eq!(code, state.to_code(),);

        assert_eq!(GameState::from_code(code), state,);
    }

    #[test]
    fn state_encoding_2() {
        use super::{Entry, GameState};

        let code = 1 + 3 + 2 * 3 * 3;

        let state = GameState {
            fields: [
                Entry::Computer,
                Entry::Computer,
                Entry::Human,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
                Entry::Empty,
            ],
        };

        assert_eq!(code, state.to_code(),);

        assert_eq!(GameState::from_code(code), state,);
    }

    #[test]
    fn state_encoding_many() {
        use super::GameState;

        let max = 19683 - 1;
        let codes = [
            0usize, 213, max, 554, 2, 345, 23, 4, 325, 43, 6536, 453, 6, 5347, 13575, 765, 537,
            6563, 5234, 9575, 4676,
        ];

        for code in codes {
            let state = GameState::from_code(code);
            assert_eq!(
                code,
                state.to_code(),
                "testing state <=> code serialisation of {state}"
            );
        }
    }

    #[test]
    fn win_move_none() {
        use super::Entry::{Human, Computer, Empty};
        use super::GameState;
        // rust-fmt disable
        let state = GameState {
            fields: [
                // 0 1 2
                Empty, Empty, Empty, // 3 4 5
                Empty, Empty, Empty, // 6 7 8
                Empty, Empty, Empty,
            ],
        };

        assert_eq!(state.get_winning_move(Human), None);
        assert_eq!(state.get_winning_move(Computer), None);
    }

    #[test]
    fn win_move1() {
        use super::Entry::{Human, Computer, Empty};
        use super::GameState;
        // rust-fmt disable
        let state = GameState {
            fields: [
                // 0 1 2
                Computer, Computer, Empty, // 3 4 5
                Empty, Human, Empty, // 6 7 8
                Empty, Human, Empty,
            ],
        };

        assert_eq!(state.get_winning_move(Human), None);
        assert_eq!(state.get_winning_move(Computer), Some(2));
    }

    #[test]
    fn win_move2() {
        use super::Entry::{Human, Computer, Empty};
        use super::GameState;
        // rust-fmt disable
        let state = GameState {
            fields: [
                // 0 1 2
                Empty, Computer, Empty, // 3 4 5
                Empty, Empty, Empty, // 6 7 8
                Computer, Empty, Computer,
            ],
        };

        assert_eq!(state.get_winning_move(Human), None);
        assert_eq!(state.get_winning_move(Computer), Some(7));
    }

    #[test]
    fn win_move1a() {
        use super::Entry::{Human, Computer, Empty};
        use super::GameState;
        // rust-fmt disable
        let state = GameState {
            fields: [
                // 0 1 2
                Computer, Computer, Empty, // 3 4 5
                Empty, Empty, Empty, // 6 7 8
                Empty, Human, Empty,
            ],
        };

        assert_eq!(state.get_winning_move(Human), None);
        assert_eq!(state.get_winning_move(Computer), Some(2));
    }
}

use std::fmt::Display;

use crate::model::Rotation;

// \in 0..9
pub type FieldId = u8;
//  0   1   2
//  3   4   5
//  6   7   8

fn rotate_id_180(f: FieldId) -> FieldId {
    match f {
        0 => 8,
        1 => 7,
        2 => 6,
        3 => 5,
        4 => 4,
        5 => 3,
        6 => 2,
        7 => 1,
        8 => 0,
        _ => unreachable!("field ids must be in interval 0..9"),
    }
}

fn rotate_id_left(f: FieldId) -> FieldId {
    match f {
        0 => 6,
        1 => 3,
        2 => 0,
        3 => 7,
        4 => 4,
        5 => 1,
        6 => 8,
        7 => 5,
        8 => 2,
        _ => unreachable!("field ids must be in interval 0..9"),
    }
}

fn rotate_id_right(f: FieldId) -> FieldId {
    match f {
        0 => 2,
        1 => 5,
        2 => 8,
        3 => 1,
        4 => 4,
        5 => 7,
        6 => 0,
        7 => 3,
        8 => 6,
        _ => unreachable!("field ids must be in interval 0..9"),
    }
}

fn flip_id(f: FieldId) -> FieldId {
    match f {
        2 | 5 | 8 => f - 2,
        0 | 3 | 6 => f + 2,
        _ => f,
    }
}

pub(crate) fn transform_id(f: FieldId, rotation: Rotation, flip: bool) -> FieldId {
    let f = if flip { flip_id(f) } else { f };

    match rotation {
        Rotation::None => f,
        Rotation::Left => rotate_id_left(f),
        Rotation::Right => rotate_id_right(f),
        Rotation::Rot180 => rotate_id_180(f),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Entry {
    Empty,
    Computer,
    Human,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GameState {
    pub fields: [Entry; 9],
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn to_char(e: Entry) -> char {
            match e {
                Entry::Empty => '.',
                Entry::Computer => 'X',
                Entry::Human => 'O',
            }
        }
        for row in 0..3 {
            let row = row * 3;
            write!(f, "|")?;
            for col in 0..3 {
                let id = row + col;
                write!(f, "{}", to_char(self.fields[id]))?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}

impl GameState {
    /// Returns the id of the field the player would need to occupy in order to win.
    /// If no such move exists, returns None
    pub fn get_winning_move(&self, player: Entry) -> Option<FieldId> {
        assert_ne!(player, Entry::Empty);

        // first check if game is already over
        if self.get_winner().is_some() {
            return None;
        }

        for i in 0..9 {
            if self.fields[i] != Entry::Empty {
                continue;
            }

            let mut state = *self;
            state.fields[i] = player;

            if state.get_winner().is_some() {
                return Some(i as u8);
            }
        }

        None
    }

    pub fn get_winner(&self) -> Option<Entry> {
        // first check each row, from top to bottom
        for row in 0..3 {
            let row = row * 3;
            let current = self.fields[row];
            // we dont need to check further, if this fields is empty anyway
            if current == Entry::Empty {
                continue;
            }

            // check if the 2 right neighbours are of the same kind
            if current == self.fields[row + 1] && self.fields[row] == self.fields[row + 2] {
                return Some(current);
            }
        }

        for col in 0..3 {
            let current = self.fields[col];
            if current == Entry::Empty {
                continue;
            }

            if current == self.fields[col + 3] && current == self.fields[col + 6] {
                return Some(current);
            }
        }

        // if the middle field is empty, we don't need to check further
        if self.fields[4] == Entry::Empty {
            return None;
        }

        if self.fields[0] == self.fields[4] && self.fields[0] == self.fields[8] {
            return Some(self.fields[0]);
        }

        if self.fields[2] == self.fields[4] && self.fields[2] == self.fields[6] {
            return Some(self.fields[2]);
        }

        None
    }

    fn map_field_indices(self, f: impl Fn(FieldId) -> FieldId) -> Self {
        let mut fields = [Entry::Empty; 9];
        for i in 0..9 {
            let new_id = f(i);
            fields[new_id as usize] = self.fields[i as usize];
        }

        Self { fields }
    }

    pub(crate) fn transform_field(self, rotation: Rotation, flip: bool) -> Self {
        let f = |field| transform_id(field, rotation, flip);
        self.map_field_indices(f)
    }

    pub fn from_code(mut code: usize) -> Self {
        let mut fields = [Entry::Empty; 9];

        for item in &mut fields {
            let rest = code % 3;
            code /= 3;

            *item = match rest {
                0 => Entry::Empty,
                1 => Entry::Computer,
                2 => Entry::Human,
                _ => unreachable!(),
            }
        }

        assert_eq!(code, 0);

        GameState { fields }
    }

    pub fn to_code(&self) -> usize {
        self.fields.iter().rev().fold(0usize, |acc, item: &Entry| {
            acc * 3
                + match item {
                    Entry::Empty => 0,
                    Entry::Computer => 1,
                    Entry::Human => 2,
                }
        })
    }

    pub fn amount_of_fields_set(&self) -> usize {
        self.fields
            .iter()
            .map(|entry| if *entry == Entry::Empty { 0 } else { 1 })
            .sum()
    }
}
