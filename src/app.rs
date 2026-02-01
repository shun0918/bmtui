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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Playing,
    Paused,
    GameOver,
    Clear,
}

pub struct App {
    should_quit: bool,
    game_state: GameState,
    app_state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            game_state: GameState::new(),
            app_state: AppState::Playing,
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

        match self.app_state {
            AppState::Paused => self.render_overlay(frame, "⏸ PAUSED", "p: 再開 | q: 終了"),
            AppState::GameOver => self.render_overlay(frame, "☠ GAME OVER", "r: リスタート | q: 終了"),
            AppState::Clear => self.render_overlay(frame, "🎉 STAGE CLEAR!", "r: リスタート | q: 終了"),
            AppState::Playing => {}
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("💣 BMTUI 💣")
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
            let mut spans = Vec::new();
            for x in 0..self.game_state.world.width() {
                let mut found = false;

                for entity in &self.game_state.entities {
                    if entity.is_alive && entity.position.x == x && entity.position.y == y {
                        let char_str = entity.to_char().to_string();
                        let span = if entity.entity_type == crate::game::entity::EntityType::Item {
                            Span::styled(char_str, Style::default().bg(Color::White).fg(Color::Black))
                        } else {
                            Span::raw(char_str)
                        };
                        spans.push(span);
                        found = true;
                        break;
                    }
                }

                if !found {
                    if let Some(tile) = self.game_state.world.get_tile(x, y) {
                        spans.push(Span::raw(tile.to_char().to_string()));
                    }
                }
            }
            lines.push(Line::from(spans));
        }

        lines
    }

    fn render_overlay(&self, frame: &mut Frame, title: &str, message: &str) {
        let area = frame.area();
        let overlay_width = 40;
        let overlay_height = 7;
        let x = (area.width.saturating_sub(overlay_width)) / 2;
        let y = (area.height.saturating_sub(overlay_height)) / 2;

        let overlay_area = Rect {
            x,
            y,
            width: overlay_width.min(area.width),
            height: overlay_height.min(area.height),
        };

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                title,
                Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(message),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black)),
            );

        frame.render_widget(paragraph, overlay_area);
    }

    pub fn handle_event(&mut self, event: crossterm::event::Event) -> bool {
        if let crossterm::event::Event::Key(key) = event {
            self.handle_key(key)
        } else {
            true
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.app_state {
            AppState::Playing => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.should_quit = true;
                    false
                }
                KeyCode::Char('p') => {
                    self.app_state = AppState::Paused;
                    true
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
            },
            AppState::Paused => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.should_quit = true;
                    false
                }
                KeyCode::Char('p') => {
                    self.app_state = AppState::Playing;
                    true
                }
                _ => true,
            },
            AppState::GameOver | AppState::Clear => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.should_quit = true;
                    false
                }
                KeyCode::Char('r') => {
                    self.game_state = GameState::new();
                    self.app_state = AppState::Playing;
                    true
                }
                _ => true,
            },
        }
    }

    pub fn tick(&mut self) {
        if self.app_state == AppState::Playing {
            self.game_state.tick(0.05);
            self.update_game_state();
        }
    }

    fn update_game_state(&mut self) {
        let player_alive = self
            .game_state
            .get_player()
            .map(|p| p.is_alive)
            .unwrap_or(false);

        let enemies_alive = self
            .game_state
            .entities
            .iter()
            .filter(|e| e.entity_type == crate::game::entity::EntityType::Enemy && e.is_alive)
            .count();

        if !player_alive {
            self.app_state = AppState::GameOver;
        } else if enemies_alive == 0 {
            self.app_state = AppState::Clear;
        }
    }
}
