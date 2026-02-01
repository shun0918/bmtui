use crate::game::{
    components::{ExplosionData, ItemType},
    entity::{Entity, EntityType},
    state::GameState,
    world::Tile,
};

pub fn update_bombs(game_state: &mut GameState, delta_time: f32) {
    let mut explosions_to_create = Vec::new();

    for entity in &mut game_state.entities {
        if entity.entity_type == EntityType::Bomb && entity.is_alive {
            if let Some(bomb_data) = &mut entity.bomb_data {
                bomb_data.timer -= delta_time;

                if bomb_data.timer <= 0.0 {
                    let x = entity.position.x;
                    let y = entity.position.y;
                    let range = bomb_data.range;
                    entity.is_alive = false;

                    explosions_to_create.push((x, y, range));
                }
            }
        }
    }

    for (x, y, range) in explosions_to_create {
        create_explosion(game_state, x, y, range);
    }
}

pub fn update_explosions(game_state: &mut GameState, delta_time: f32) {
    for entity in &mut game_state.entities {
        if entity.entity_type == EntityType::Explosion && entity.is_alive {
            if let Some(explosion_data) = &mut entity.explosion_data {
                explosion_data.timer -= delta_time;

                if explosion_data.timer <= 0.0 {
                    entity.is_alive = false;
                }
            }
        }
    }
}

fn create_explosion(game_state: &mut GameState, x: usize, y: usize, range: usize) {
    game_state.add_explosion(x, y);

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for (dx, dy) in &directions {
        for i in 1..=range {
            let new_x = (x as i32 + dx * i as i32) as usize;
            let new_y = (y as i32 + dy * i as i32) as usize;

            if let Some(tile) = game_state.world.get_tile(new_x, new_y) {
                match tile {
                    Tile::Wall => break,
                    Tile::Breakable => {
                        game_state.world.set_tile(new_x, new_y, Tile::Empty);
                        game_state.add_explosion(new_x, new_y);

                        if rand::random::<f32>() < 0.3 {
                            let item_type = if rand::random::<bool>() {
                                ItemType::Fire
                            } else {
                                ItemType::Bomb
                            };
                            game_state.add_item(new_x, new_y, item_type);
                        }
                        break;
                    }
                    Tile::Empty => {
                        game_state.add_explosion(new_x, new_y);
                    }
                }
            } else {
                break;
            }
        }
    }

    damage_entities_in_explosions(game_state);
}

fn damage_entities_in_explosions(game_state: &mut GameState) {
    let explosion_positions: Vec<(usize, usize)> = game_state
        .entities
        .iter()
        .filter(|e| e.entity_type == EntityType::Explosion && e.is_alive)
        .map(|e| (e.position.x, e.position.y))
        .collect();

    for entity in &mut game_state.entities {
        if entity.is_alive
            && (entity.entity_type == EntityType::Player || entity.entity_type == EntityType::Enemy)
        {
            for (ex, ey) in &explosion_positions {
                if entity.position.x == *ex && entity.position.y == *ey {
                    entity.is_alive = false;
                    break;
                }
            }
        }
    }
}
