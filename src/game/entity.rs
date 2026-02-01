use super::components::{BombData, ExplosionData, ItemType, PlayerStats, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Enemy,
    Bomb,
    Explosion,
    Item,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: usize,
    pub entity_type: EntityType,
    pub position: Position,
    pub player_stats: Option<PlayerStats>,
    pub bomb_data: Option<BombData>,
    pub explosion_data: Option<ExplosionData>,
    pub item_type: Option<ItemType>,
    pub is_alive: bool,
}

impl Entity {
    pub fn new_player(id: usize, x: usize, y: usize) -> Self {
        Self {
            id,
            entity_type: EntityType::Player,
            position: Position::new(x, y),
            player_stats: Some(PlayerStats::default()),
            bomb_data: None,
            explosion_data: None,
            item_type: None,
            is_alive: true,
        }
    }

    pub fn new_enemy(id: usize, x: usize, y: usize) -> Self {
        Self {
            id,
            entity_type: EntityType::Enemy,
            position: Position::new(x, y),
            player_stats: None,
            bomb_data: None,
            explosion_data: None,
            item_type: None,
            is_alive: true,
        }
    }

    pub fn new_bomb(id: usize, x: usize, y: usize, range: usize, owner_id: usize) -> Self {
        Self {
            id,
            entity_type: EntityType::Bomb,
            position: Position::new(x, y),
            player_stats: None,
            bomb_data: Some(BombData::new(range, owner_id)),
            explosion_data: None,
            item_type: None,
            is_alive: true,
        }
    }

    pub fn new_explosion(id: usize, x: usize, y: usize) -> Self {
        Self {
            id,
            entity_type: EntityType::Explosion,
            position: Position::new(x, y),
            player_stats: None,
            bomb_data: None,
            explosion_data: Some(ExplosionData::new()),
            item_type: None,
            is_alive: true,
        }
    }

    pub fn new_item(id: usize, x: usize, y: usize, item_type: ItemType) -> Self {
        Self {
            id,
            entity_type: EntityType::Item,
            position: Position::new(x, y),
            player_stats: None,
            bomb_data: None,
            explosion_data: None,
            item_type: Some(item_type),
            is_alive: true,
        }
    }

    pub fn to_char(&self) -> &str {
        match self.entity_type {
            EntityType::Player => "🧑",
            EntityType::Enemy => "👾",
            EntityType::Bomb => "💣",
            EntityType::Explosion => "💥",
            EntityType::Item => {
                if let Some(item_type) = &self.item_type {
                    item_type.to_char()
                } else {
                    "?"
                }
            }
        }
    }
}
