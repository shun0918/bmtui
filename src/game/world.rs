use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Breakable,
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    pub fn to_char(&self) -> &str {
        match self {
            Tile::Empty => "  ",
            Tile::Wall => "🧱",
            Tile::Breakable => "📦",
        }
    }
}

pub struct World {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Empty; width]; height];
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn from_layout(layout: &[&str]) -> Self {
        let height = layout.len();
        let width = if height > 0 { layout[0].len() } else { 0 };

        let mut tiles = vec![vec![Tile::Empty; width]; height];

        for (y, row) in layout.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                tiles[y][x] = match ch {
                    '#' => Tile::Wall,
                    'X' => Tile::Breakable,
                    _ => Tile::Empty,
                };
            }
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y)
            .map(|t| t.is_walkable())
            .unwrap_or(false)
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile.to_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
