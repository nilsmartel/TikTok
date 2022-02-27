#[cfg(test)]
mod tests {
    #[test]
    fn rotate_right() {
        use super::rotate_id_right as right;
        for i in 0..9 {
            assert_eq!(i, right(right(right(i))));
        }
    }

    #[test]
    fn rotate_left() {
        use super::rotate_id_left as left;
        for i in 0..9 {
            assert_eq!(i, left(left(left(i))));
        }
    }

    #[test]
    fn rotate_180() {
        use super::rotate_id_180 as rot;
        for i in 0..9 {
            assert_eq!(i, rot(rot(i)));
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
            assert_eq!(code, State::from_code(code).to_code());
        }
    }

    #[test]
    fn state_setup() {
        use super::Game;

        let game = Game::new();
        assert_eq!(game.states.len(), 304);
    }
}

use std::collections::HashMap;

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

impl State {
    fn get_winner(&self) -> Option<Entry> {
        for row in 0..3 {
            let row = row * 3;
            if self.fields[row] == Entry::Empty {
                continue;
            }

            if self.fields[row] == self.fields[row + 1] && self.fields[row] == self.fields[row + 2]
            {
                return Some(self.fields[row]);
            }
        }

        for col in 0..3 {
            if self.fields[col] == Entry::Empty {
                continue;
            }

            if self.fields[col] == self.fields[col + 3] && self.fields[col] == self.fields[col + 6]
            {
                return Some(self.fields[col]);
            }
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

    pub fn new() -> Game {
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
            let options = state
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
                .collect();

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
