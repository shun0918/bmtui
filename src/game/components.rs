#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerStats {
    pub max_bombs: usize,
    pub bomb_range: usize,
    pub speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            max_bombs: 1,
            bomb_range: 1,
            speed: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BombData {
    pub timer: f32,
    pub range: usize,
    pub owner_id: usize,
}

impl BombData {
    pub fn new(range: usize, owner_id: usize) -> Self {
        Self {
            timer: 3.0,
            range,
            owner_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExplosionData {
    pub timer: f32,
}

impl ExplosionData {
    pub fn new() -> Self {
        Self { timer: 0.5 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Fire,
    Bomb,
}

impl ItemType {
    pub fn to_char(&self) -> &str {
        match self {
            ItemType::Fire => "🔥",
            ItemType::Bomb => "💣",
        }
    }
}
