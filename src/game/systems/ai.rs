use crate::game::{
    components::Direction,
    entity::EntityType,
    state::GameState,
};
use rand::Rng;

pub fn update_enemy_ai(game_state: &mut GameState) {
    let enemy_ids: Vec<usize> = game_state
        .entities
        .iter()
        .filter(|e| e.entity_type == EntityType::Enemy && e.is_alive)
        .map(|e| e.id)
        .collect();

    for enemy_id in enemy_ids {
        random_walk_ai(game_state, enemy_id);
    }
}

fn random_walk_ai(game_state: &mut GameState, enemy_id: usize) {
    let mut rng = rand::thread_rng();
    let direction = match rng.gen_range(0..4) {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        _ => Direction::Right,
    };

    game_state.move_entity(enemy_id, direction);
}

pub fn check_player_enemy_collision(game_state: &mut GameState) {
    let player_pos = if let Some(player) = game_state.get_player() {
        if player.is_alive {
            Some((player.position.x, player.position.y))
        } else {
            None
        }
    } else {
        None
    };

    if let Some((px, py)) = player_pos {
        for entity in &game_state.entities {
            if entity.entity_type == EntityType::Enemy
                && entity.is_alive
                && entity.position.x == px
                && entity.position.y == py
            {
                if let Some(player) = game_state.get_player_mut() {
                    player.is_alive = false;
                }
                break;
            }
        }
    }
}
