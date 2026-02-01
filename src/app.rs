use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crossterm::event::{KeyCode, KeyEvent};

pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        self.render_header(frame, chunks[0]);
        self.render_game_board(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("💣 Bomberman TUI 💣")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, area);
    }

    fn render_game_board(&self, frame: &mut Frame, area: Rect) {
        let board = self.create_sample_board();
        let paragraph = Paragraph::new(board)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Game Board"));
        frame.render_widget(paragraph, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let help_text = Line::from(vec![
            Span::raw("hjkl: 移動 | "),
            Span::raw("Space: 爆弾設置 | "),
            Span::raw("p: ポーズ | "),
            Span::styled("q: 終了", Style::default().fg(Color::Red)),
        ]);
        let paragraph = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(paragraph, area);
    }

    fn create_sample_board(&self) -> Vec<Line<'static>> {
        vec![
            Line::from("###############"),
            Line::from("#🧑   📦   📦  👾#"),
            Line::from("# 🧱 # 🧱 # 🧱 # 🧱 #"),
            Line::from("#   📦   📦   📦#"),
            Line::from("# 🧱 # 🧱 # 🧱 # 🧱 #"),
            Line::from("#   📦   📦   📦#"),
            Line::from("# 🧱 # 🧱 # 🧱 # 🧱 #"),
            Line::from("#   📦   📦   📦#"),
            Line::from("# 🧱 # 🧱 # 🧱 # 🧱 #"),
            Line::from("#👾  📦   📦   👾#"),
            Line::from("###############"),
        ]
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
            _ => true,
        }
    }

    pub fn tick(&mut self) {
    }
}
