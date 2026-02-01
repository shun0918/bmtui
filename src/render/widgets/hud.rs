use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::{entity::EntityType, GameState};

pub fn render_hud(frame: &mut Frame, area: Rect, game_state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    render_player_stats(frame, chunks[0], game_state);
    render_game_info(frame, chunks[1], game_state);
    render_controls(frame, chunks[2]);
}

fn render_player_stats(frame: &mut Frame, area: Rect, game_state: &GameState) {
    let player = game_state.get_player();
    let stats = player.and_then(|p| p.player_stats.as_ref());

    let max_bombs = stats.map(|s| s.max_bombs).unwrap_or(0);
    let bomb_range = stats.map(|s| s.bomb_range).unwrap_or(0);

    let text = vec![
        Line::from(vec![
            Span::styled("💣: ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{}", max_bombs)),
        ]),
        Line::from(vec![
            Span::styled("🔥: ", Style::default().fg(Color::Red)),
            Span::raw(format!("{}", bomb_range)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Stats"))
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn render_game_info(frame: &mut Frame, area: Rect, game_state: &GameState) {
    let enemies_alive = game_state
        .entities
        .iter()
        .filter(|e| e.entity_type == EntityType::Enemy && e.is_alive)
        .count();

    let player_alive = game_state
        .get_player()
        .map(|p| p.is_alive)
        .unwrap_or(false);

    let status = if !player_alive {
        Span::styled("GAME OVER", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
    } else if enemies_alive == 0 {
        Span::styled("CLEAR!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(format!("敵: {}", enemies_alive), Style::default().fg(Color::Cyan))
    };

    let text = vec![Line::from(status)];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_controls(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("hjkl: 移動"),
        Line::from("Space: 爆弾"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}
