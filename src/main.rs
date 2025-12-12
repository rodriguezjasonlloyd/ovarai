mod app;
mod updater;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{
    Action, AnalyzeMenuItem, App, ExperimentMenuItem, MainMenuItem, Screen, ShowcaseMenuItem,
};

fn render(frame: &mut Frame, app: &App) -> () {
    match app.screen {
        Screen::MainMenu => render_main_menu(frame, app),
        Screen::AnalyzeMenu => render_analyze_menu(frame, app),
        Screen::ExperimentMenu => render_experiment_menu(frame, app),
        Screen::ShowcaseMenu => render_showcase_menu(frame, app),
    }
}

fn render_main_menu(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let list = List::new(
        MainMenuItem::all()
            .iter()
            .map(|item| {
                let content = format!("{:?}", item);
                let style = if *item == app.selected_main {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect::<Vec<ListItem>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Line::from("Main Menu").centered()),
    );

    frame.render_widget(list, chunks[0]);

    let footer = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | q: Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, chunks[1]);
}

fn render_analyze_menu(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let list = List::new(
        AnalyzeMenuItem::all()
            .iter()
            .map(|item| {
                let content = format!("{:?}", item);
                let style = if *item == app.selected_analyze {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect::<Vec<ListItem>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Line::from("Analyze").centered()),
    );

    frame.render_widget(list, chunks[0]);

    let footer = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | q: Back")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, chunks[1]);
}

fn render_experiment_menu(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let list = List::new(
        ExperimentMenuItem::all()
            .iter()
            .map(|item| {
                let content = format!("{:?}", item);
                let style = if *item == app.selected_experiment {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect::<Vec<ListItem>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Line::from("Experiment").centered()),
    );

    frame.render_widget(list, chunks[0]);

    let footer = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | q: Back")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, chunks[1]);
}

fn render_showcase_menu(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let list = List::new(
        ShowcaseMenuItem::all()
            .iter()
            .map(|item| {
                let content = format!("{:?}", item);
                let style = if *item == app.selected_showcase {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect::<Vec<ListItem>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Line::from("Showcase").centered()),
    );

    frame.render_widget(list, chunks[0]);

    let footer = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | q: Back")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, chunks[1]);
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            render(frame, &app);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.handle_key(key.code) {
                    Action::Continue => {}
                    Action::Quit => break,
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    updater::check_and_update()?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}
