use super::{
    components::{Direction, ItemType},
    entity::{Entity, EntityType},
    systems::ai::{check_player_enemy_collision, update_enemy_ai},
    systems::bomb::{update_bombs, update_explosions},
    world::World,
};

pub struct GameState {
    pub world: World,
    pub entities: Vec<Entity>,
    next_entity_id: usize,
    pub player_id: usize,
    enemy_move_timer: f32,
}

impl GameState {
    pub fn new() -> Self {
        let layout = vec![
            "###############",
            "#P   X   X   E#",
            "# # # # # # # #",
            "#   X   X   X #",
            "# # # # # # # #",
            "#   X   X   X #",
            "# # # # # # # #",
            "#   X   X   X #",
            "# # # # # # # #",
            "#E  X   X   E #",
            "###############",
        ];

        let mut world = World::from_layout(&layout);
        let mut entities = Vec::new();
        let mut next_entity_id = 0;
        let mut player_id = 0;

        for (y, row) in layout.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    'P' => {
                        player_id = next_entity_id;
                        entities.push(Entity::new_player(next_entity_id, x, y));
                        next_entity_id += 1;
                    }
                    'E' => {
                        entities.push(Entity::new_enemy(next_entity_id, x, y));
                        next_entity_id += 1;
                    }
                    _ => {}
                }
            }
        }

        Self {
            world,
            entities,
            next_entity_id,
            player_id,
            enemy_move_timer: 0.0,
        }
    }

    pub fn get_player(&self) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == self.player_id)
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == self.player_id)
    }

    pub fn move_entity(&mut self, entity_id: usize, direction: Direction) -> bool {
        let entity = self
            .entities
            .iter()
            .find(|e| e.id == entity_id && e.is_alive);

        if let Some(entity) = entity {
            let (dx, dy) = direction.to_delta();
            let new_x = (entity.position.x as i32 + dx) as usize;
            let new_y = (entity.position.y as i32 + dy) as usize;

            if self.can_move_to(new_x, new_y, entity_id) {
                if let Some(entity) = self.entities.iter_mut().find(|e| e.id == entity_id) {
                    entity.position.x = new_x;
                    entity.position.y = new_y;
                    return true;
                }
            }
        }

        false
    }

    fn can_move_to(&self, x: usize, y: usize, entity_id: usize) -> bool {
        if !self.world.is_walkable(x, y) {
            return false;
        }

        for entity in &self.entities {
            if entity.id != entity_id
                && entity.is_alive
                && entity.position.x == x
                && entity.position.y == y
                && matches!(entity.entity_type, EntityType::Bomb)
            {
                return false;
            }
        }

        true
    }

    pub fn place_bomb(&mut self) -> bool {
        if let Some(player) = self.get_player() {
            if !player.is_alive {
                return false;
            }

            let x = player.position.x;
            let y = player.position.y;
            let range = player
                .player_stats
                .as_ref()
                .map(|s| s.bomb_range)
                .unwrap_or(1);

            let active_bombs = self
                .entities
                .iter()
                .filter(|e| {
                    e.entity_type == EntityType::Bomb
                        && e.is_alive
                        && e.bomb_data.as_ref().map(|b| b.owner_id == self.player_id).unwrap_or(false)
                })
                .count();

            let max_bombs = self
                .get_player()
                .and_then(|p| p.player_stats.as_ref())
                .map(|s| s.max_bombs)
                .unwrap_or(1);

            if active_bombs >= max_bombs {
                return false;
            }

            for entity in &self.entities {
                if entity.is_alive
                    && entity.position.x == x
                    && entity.position.y == y
                    && entity.entity_type == EntityType::Bomb
                {
                    return false;
                }
            }

            let bomb = Entity::new_bomb(self.next_entity_id, x, y, range, self.player_id);
            self.next_entity_id += 1;
            self.entities.push(bomb);
            true
        } else {
            false
        }
    }

    pub fn add_explosion(&mut self, x: usize, y: usize) {
        let explosion = Entity::new_explosion(self.next_entity_id, x, y);
        self.next_entity_id += 1;
        self.entities.push(explosion);
    }

    pub fn add_item(&mut self, x: usize, y: usize, item_type: ItemType) {
        let item = Entity::new_item(self.next_entity_id, x, y, item_type);
        self.next_entity_id += 1;
        self.entities.push(item);
    }

    fn collect_items(&mut self) {
        let player_pos = if let Some(player) = self.get_player() {
            if player.is_alive {
                Some((player.position.x, player.position.y))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((px, py)) = player_pos {
            let mut items_to_collect = Vec::new();

            for entity in &self.entities {
                if entity.entity_type == EntityType::Item
                    && entity.is_alive
                    && entity.position.x == px
                    && entity.position.y == py
                {
                    if let Some(item_type) = entity.item_type {
                        items_to_collect.push((entity.id, item_type));
                    }
                }
            }

            for (item_id, item_type) in items_to_collect {
                if let Some(player) = self.get_player_mut() {
                    if let Some(stats) = &mut player.player_stats {
                        match item_type {
                            ItemType::Fire => {
                                stats.bomb_range += 1;
                            }
                            ItemType::Bomb => {
                                stats.max_bombs += 1;
                            }
                        }
                    }
                }

                if let Some(item) = self.entities.iter_mut().find(|e| e.id == item_id) {
                    item.is_alive = false;
                }
            }
        }
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.enemy_move_timer += delta_time;
        if self.enemy_move_timer >= 0.3 {
            update_enemy_ai(self);
            self.enemy_move_timer = 0.0;
        }

        update_bombs(self, delta_time);
        update_explosions(self, delta_time);
        self.collect_items();
        check_player_enemy_collision(self);
    }
}
