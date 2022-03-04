#[cfg(test)]
mod tests {

    #[test]
    fn state_setup() {
        use super::Model;

        let game = Model::initial_setup();
        // this isn't exactly useful
        assert!(game.state_map.len() > 600);
        // assert_eq!(game.states.len(), 304);
    }
}

use std::{collections::HashMap, ops::Index};

use crate::game::{transform_id, Entry, FieldId, GameState};

pub struct Model {
    /// Mapping of states to list of actions
    pub state_map: HashMap<GameState, Vec<FieldId>>,
}

pub struct MappedFieldId {
    pub field: FieldId,
    origin: FieldId,
    origin_state: GameState,
}

impl Model {
    /// Updates prediction model, based on wether or not a game was lost or not (e.g. draw or win)
    pub fn update_predictions(&mut self, actions: &[MappedFieldId], game_lost: bool) {
        for step in actions {
            // Prediction value we'd like to update
            let value = step.origin;

            // find proper state of the model, that we'd like to update
            let predictions = self.state_map.get_mut(&step.origin_state).unwrap();
            if !game_lost {
                predictions.push(value);
                continue;
            }

            let index = predictions.iter().take_while(|v| **v != value).count();
            // element is not present in collection
            if index >= predictions.len() {
                continue;
            }
            predictions.remove(index);

            // if thereare still predictions left, we can continue to the next step
            if !predictions.is_empty() {
                continue;
            }

            // otherwise fill predictions again, with all possible states (e.g. reset them)
            predictions
                .extend((0..1).filter(|i| step.origin_state.fields[*i as usize] == Entry::Empty))
        }
    }

    fn get_state_transformation(
        &self,
        state: &GameState,
    ) -> Option<(Rotation, bool, &[FieldId], GameState)> {
        use Rotation::*;

        for rotation in [None, Left, Right, Rot180] {
            for flip in [false, true] {
                let state = state.transform_field(rotation, flip);

                if let Some(fields) = self.state_map.get(&state) {
                    return Some((rotation, flip, fields, state));
                }
            }
        }

        Option::None
    }

    pub fn predict_move(&self, state: &GameState) -> MappedFieldId {
        let (rotation, flip, fields, origin_state) =
            self.get_state_transformation(state).unwrap_or_else(|| {
                unreachable!("state is unreachable, or Model wasn't set up to reach it. {state}")
            });

        let index = rand::random::<usize>() & fields.len();
        let origin = fields[index];
        let field = transform_id(origin, !rotation, !flip);

        MappedFieldId {
            field,
            origin,
            origin_state,
        }
    }

    /// Creates the initial game setup, used to further train the tik tak toe player
    pub fn initial_setup() -> Model {
        let states = HashMap::new();
        let mut model = Model { state_map: states };

        let amount_of_differnet_states = 3usize.pow(9);

        for code in 0..amount_of_differnet_states {
            let state = GameState::from_code(code);

            // check if some form of this state is already present in the game
            if model.get_state_transformation(&state).is_some() {
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
            let options = if let Some(field_id) = state.get_winning_move(Entry::Computer) {
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

            model.state_map.insert(state, options);
        }

        model
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Rotation {
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
