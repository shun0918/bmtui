use crate::game::{components::Direction, state::GameState};

pub fn process_movement(game_state: &mut GameState, entity_id: usize, direction: Direction) {
    game_state.move_entity(entity_id, direction);
}
