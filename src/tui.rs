use crate::api;
use crate::auth::Token;
use crate::config::{self, Config};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::io::{self, Write};

enum MenuItem {
    Devices,
    Config,
}

pub async fn run(config: Config, token: Token) -> Result<()> {
    // Setup terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut menu_item = MenuItem::Devices;
    let mut devices: Vec<api::Device> = Vec::new();

    // Fetch devices
    let config_clone = config.clone();
    let token_clone = token.clone();
    let devices_result = api::get_all_devices(&config_clone, &token_clone).await;
    if let Ok(devs) = devices_result {
        devices = devs;
    } else {
        eprintln!("Failed to fetch devices: {}", devices_result.err().unwrap());
    }

    // Initialize menu state
    let mut menu_state = ListState::default();
    menu_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                )
                .split(size);

            // Menu
            let menu_items = vec![ListItem::new("Devices"), ListItem::new("Config")];
            let menu = List::new(menu_items)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(menu, chunks[0], &mut menu_state);

            // Main area
            match menu_item {
                MenuItem::Devices => {
                    let items: Vec<ListItem> = devices
                        .iter()
                        .map(|device| {
                            let content = vec![Spans::from(format!(
                                "{} - {}",
                                device.hostname.as_deref().unwrap_or("N/A"),
                                device
                                    .managementIpAddress
                                    .as_deref()
                                    .unwrap_or("N/A")
                            ))];
                            ListItem::new(content).style(Style::default())
                        })
                        .collect();

                    let devices_list = List::new(items)
                        .block(Block::default().title("Devices").borders(Borders::ALL))
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                    f.render_widget(devices_list, chunks[1]);
                }
                MenuItem::Config => {
                    let config_items = vec![ListItem::new("Reset Configuration")];
                    let config_list = List::new(config_items)
                        .block(Block::default().title("Config").borders(Borders::ALL))
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                    f.render_widget(config_list, chunks[1]);
                }
            }
        })?;

        // Handle input
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let i = match menu_state.selected() {
                            Some(i) => {
                                if i >= 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        menu_state.select(Some(i));
                        menu_item = match i {
                            0 => MenuItem::Devices,
                            1 => MenuItem::Config,
                            _ => MenuItem::Devices,
                        };
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        let i = match menu_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    1
                                } else {
                                    i - 1
                                }
                            }
                            None => 1,
                        };
                        menu_state.select(Some(i));
                        menu_item = match i {
                            0 => MenuItem::Devices,
                            1 => MenuItem::Config,
                            _ => MenuItem::Devices,
                        };
                    }
                    KeyCode::Enter => {
                        if let MenuItem::Config = menu_item {
                            // Confirm reset
                            println!("Are you sure you want to reset the configuration? (y/n): ");
                            io::stdout().flush()?;
                            if let Event::Key(key) = event::read()? {
                                if let KeyCode::Char('y') = key.code {
                                    config::reset_config()?;
                                    println!("Configuration reset successfully.");
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
