use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Inbox => render_inbox(f),
        CurrentScreen::Compose => render_compose(f),
        CurrentScreen::Exiting => render_exiting(f),
    }
}

fn render_inbox(f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.size());

    let emails = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100 / 5); 5])
        .split(layout[0]);

    for (index, _) in emails.iter().enumerate() {
        let title = format!("Email {}", index + 1);

        f.render_widget(
            Block::default().title(title).borders(Borders::ALL),
            emails[index],
        );
    }

    f.render_widget(
        Block::new().borders(Borders::ALL).title("Message"),
        layout[1],
    );
}

fn render_compose(f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(f.size());

    f.render_widget(Block::new().borders(Borders::ALL).title("To"), layout[0]);

    f.render_widget(
        Block::new().borders(Borders::ALL).title("Subject"),
        layout[1],
    );

    f.render_widget(
        Block::new().borders(Borders::ALL).title("Message"),
        layout[2],
    );
}

fn render_exiting(f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let emails_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[0]);

    f.render_widget(
        Block::new().borders(Borders::ALL).title("Exiting"),
        emails_layout[0],
    );

    f.render_widget(
        Block::new().borders(Borders::ALL).title("Email 1"),
        emails_layout[1],
    );

    f.render_widget(
        Block::new().borders(Borders::ALL).title("Message"),
        layout[1],
    );
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
