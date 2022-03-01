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
    fn state_encoding() {
        use super::State;

        let codes = [
            0usize, 213, 231312, 554, 2, 345, 23, 4, 325, 43, 65436, 453, 6, 5347, 163575, 765,
            537, 658567463, 5234, 95785, 546676,
        ];

        for code in codes {
            let state = State::from_code(code);
            assert_eq!(
                code,
                state.to_code(),
                "testing state <=> code serialisation of {state}"
            );
        }
    }

    #[test]
    fn win_move_none() {
        use super::Entry::{Circle, Cross, Empty};
        use super::State;
        // rust-fmt disable
        let state = State {
            fields: [
                // 0 1 2
                Empty, Empty, Empty, // 3 4 5
                Empty, Empty, Empty, // 6 7 8
                Empty, Empty, Empty,
            ],
        };

        assert_eq!(state.get_winning_move(Circle), None);
        assert_eq!(state.get_winning_move(Cross), None);
    }

    #[test]
    fn win_move1() {
        use super::Entry::{Circle, Cross, Empty};
        use super::State;
        // rust-fmt disable
        let state = State {
            fields: [
                // 0 1 2
                Cross, Cross, Empty, // 3 4 5
                Empty, Circle, Empty, // 6 7 8
                Empty, Circle, Empty,
            ],
        };

        assert_eq!(state.get_winning_move(Circle), None);
        assert_eq!(state.get_winning_move(Cross), Some(2));
    }

    #[test]
    fn win_move2() {
        use super::Entry::{Circle, Cross, Empty};
        use super::State;
        // rust-fmt disable
        let state = State {
            fields: [
                // 0 1 2
                Empty, Cross, Empty, // 3 4 5
                Empty, Empty, Empty, // 6 7 8
                Cross, Empty, Cross,
            ],
        };

        assert_eq!(state.get_winning_move(Circle), None);
        assert_eq!(state.get_winning_move(Cross), Some(7));
    }

    #[test]
    fn win_move1a() {
        use super::Entry::{Circle, Cross, Empty};
        use super::State;
        // rust-fmt disable
        let state = State {
            fields: [
                // 0 1 2
                Cross, Cross, Empty, // 3 4 5
                Empty, Empty, Empty, // 6 7 8
                Empty, Circle, Empty,
            ],
        };

        assert_eq!(state.get_winning_move(Circle), None);
        assert_eq!(state.get_winning_move(Cross), Some(2));
    }

    #[test]
    fn state_setup() {
        use super::Game;

        let game = Game::initial_setup(super::Entry::Cross);
        // this isn't exactly useful
        assert!(game.states.len() > 600);
        // assert_eq!(game.states.len(), 304);
    }
}

use std::{collections::HashMap, fmt::Display, io::Empty};

// \in 0..9
type FieldId = u8;
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

fn transform_id(f: FieldId, rotation: Rotation, flip: bool) -> FieldId {
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
    Cross,
    Circle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct State {
    fields: [Entry; 9],
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn to_char(e: Entry) -> char {
            match e {
                Entry::Empty => '.',
                Entry::Cross => 'X',
                Entry::Circle => 'O',
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

impl State {
    /// Returns the id of the field the player would need to occupy in order to win.
    /// If no such move exists, returns None
    fn get_winning_move(&self, player: Entry) -> Option<FieldId> {
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

    fn get_winner(&self) -> Option<Entry> {
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

    fn transmute(self, f: impl Fn(FieldId) -> FieldId) -> Self {
        let mut fields = [Entry::Empty; 9];
        for i in 0..9 {
            let new_id = f(i);
            fields[new_id as usize] = self.fields[i as usize];
        }

        Self { fields }
    }

    fn transform(self, rotation: Rotation, flip: bool) -> Self {
        let f = |field| transform_id(field, rotation, flip);
        self.transmute(f)
    }

    pub fn from_code(mut code: usize) -> Self {
        let mut fields = [Entry::Empty; 9];

        for item in &mut fields {
            let rest = code % 3;
            code /= 3;

            *item = match rest {
                0 => Entry::Empty,
                1 => Entry::Cross,
                2 => Entry::Circle,
                _ => unreachable!(),
            }
        }

        assert_eq!(code, 0);

        State { fields }
    }

    pub fn to_code(&self) -> usize {
        self.fields.iter().rev().fold(0usize, |acc, item: &Entry| {
            acc * 3
                + match item {
                    Entry::Empty => 0,
                    Entry::Cross => 1,
                    Entry::Circle => 2,
                }
        })
    }

    fn amount_of_fields_set(&self) -> usize {
        self.fields
            .iter()
            .map(|entry| if *entry == Entry::Empty { 0 } else { 1 })
            .sum()
    }
}

pub struct Game {
    pub states: HashMap<State, Vec<FieldId>>,
}

impl Game {
    fn get_available_moves(&self, state: &State) -> Option<(&[FieldId], Rotation, bool)> {
        use Rotation::*;

        for rotation in [None, Left, Right, Rot180] {
            for flip in [false, true] {
                let state = state.transform(rotation, flip);

                if let Some(field_ids) = self.states.get(&state) {
                    return Some((field_ids, rotation, flip));
                }
            }
        }

        Option::None
    }

    /// Creates the initial game setup, used to further train the tik tak toe player
    pub fn initial_setup(cpu_player: Entry) -> Game {
        assert_ne!(cpu_player, Entry::Empty);

        let states = HashMap::new();
        let mut game = Game { states };

        let amount_of_differnet_states = 3usize.pow(9);

        for code in 0..amount_of_differnet_states {
            let state = State::from_code(code);

            // check if some form of this state is already present in the game
            if game.get_available_moves(&state).is_some() {
                continue;
            }

            // filter out all states, were someone has won
            if state.get_winner().is_some() {
                continue;
            }

            // filter out all states, were Player 1 (AI) is not picking
            // (also filters out all states, were no moves are left)
            if state.amount_of_fields_set() % 2 == 0 {
                continue;
            }

            // create a set of actions we can take from this state,
            // filtering out all illegal moves
            // or, if there is an obvious winning scenarion, pick that one
            let options = if let Some(field_id) = state.get_winning_move(cpu_player) {
                vec![field_id]
            } else {
                state
                    .fields
                    .iter()
                    .enumerate()
                    .filter_map(|(i, e)| {
                        // The FieldId represents a possible next state to fill.
                        // only empty fields can be filled in tik tak to
                        if *e == Entry::Empty {
                            Some(i as FieldId)
                        } else {
                            None
                        }
                    })
                    .collect()
            };

            game.states.insert(state, options);
        }

        game
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rotation {
    None,
    Left,
    Right,
    Rot180,
}

// Implementing the inverse operation, the way it is implemented for booleans
impl std::ops::Not for Rotation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            x => x,
        }
    }
}
