use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::game::{components::Direction as GameDirection, GameState};
use crate::render::widgets::hud;

pub struct App {
    should_quit: bool,
    game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            game_state: GameState::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(5),
            ])
            .split(frame.area());

        self.render_header(frame, chunks[0]);
        self.render_game_board(frame, chunks[1]);
        hud::render_hud(frame, chunks[2], &self.game_state);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("💣 Bomberman TUI 💣")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, area);
    }

    fn render_game_board(&self, frame: &mut Frame, area: Rect) {
        let board = self.create_game_board();
        let paragraph = Paragraph::new(board)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Game Board"));
        frame.render_widget(paragraph, area);
    }

    fn create_game_board(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        for y in 0..self.game_state.world.height() {
            let mut line_str = String::new();
            for x in 0..self.game_state.world.width() {
                let mut found = false;

                for entity in &self.game_state.entities {
                    if entity.is_alive && entity.position.x == x && entity.position.y == y {
                        line_str.push_str(entity.to_char());
                        found = true;
                        break;
                    }
                }

                if !found {
                    if let Some(tile) = self.game_state.world.get_tile(x, y) {
                        line_str.push_str(tile.to_char());
                    }
                }
            }
            lines.push(Line::from(line_str));
        }

        lines
    }

    pub fn handle_event(&mut self, event: crossterm::event::Event) -> bool {
        if let crossterm::event::Event::Key(key) = event {
            self.handle_key(key)
        } else {
            true
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
                false
            }
            KeyCode::Char('h') => {
                self.game_state
                    .move_entity(self.game_state.player_id, GameDirection::Left);
                true
            }
            KeyCode::Char('j') => {
                self.game_state
                    .move_entity(self.game_state.player_id, GameDirection::Down);
                true
            }
            KeyCode::Char('k') => {
                self.game_state
                    .move_entity(self.game_state.player_id, GameDirection::Up);
                true
            }
            KeyCode::Char('l') => {
                self.game_state
                    .move_entity(self.game_state.player_id, GameDirection::Right);
                true
            }
            KeyCode::Char(' ') => {
                self.game_state.place_bomb();
                true
            }
            _ => true,
        }
    }

    pub fn tick(&mut self) {
        self.game_state.tick(0.05);
    }
}
