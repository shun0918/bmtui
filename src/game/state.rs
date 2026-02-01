use super::{
    components::Direction,
    entity::{Entity, EntityType},
    world::World,
};

pub struct GameState {
    pub world: World,
    pub entities: Vec<Entity>,
    next_entity_id: usize,
    pub player_id: usize,
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

    pub fn tick(&mut self, delta_time: f32) {
    }
}
