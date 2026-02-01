use crate::game::{
    components::ItemType,
    entity::EntityType,
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
    let mut pending = vec![(x, y, range)];

    while let Some((cx, cy, crange)) = pending.pop() {
        game_state.add_explosion(cx, cy);

        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in &directions {
            for i in 1..=crange {
                let new_x = (cx as i32 + dx * i as i32) as usize;
                let new_y = (cy as i32 + dy * i as i32) as usize;

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

        let chain = damage_entities_in_explosions(game_state);
        pending.extend(chain);
    }
}

fn damage_entities_in_explosions(game_state: &mut GameState) -> Vec<(usize, usize, usize)> {
    let explosion_positions: Vec<(usize, usize)> = game_state
        .entities
        .iter()
        .filter(|e| e.entity_type == EntityType::Explosion && e.is_alive)
        .map(|e| (e.position.x, e.position.y))
        .collect();

    let mut chain_explosions = Vec::new();

    for entity in &mut game_state.entities {
        if !entity.is_alive {
            continue;
        }

        for (ex, ey) in &explosion_positions {
            if entity.position.x != *ex || entity.position.y != *ey {
                continue;
            }

            match entity.entity_type {
                EntityType::Player | EntityType::Enemy => {
                    entity.is_alive = false;
                }
                EntityType::Bomb => {
                    entity.is_alive = false;
                    if let Some(bomb_data) = &entity.bomb_data {
                        chain_explosions.push((entity.position.x, entity.position.y, bomb_data.range));
                    }
                }
                _ => {}
            }
            break;
        }
    }

    chain_explosions
}
