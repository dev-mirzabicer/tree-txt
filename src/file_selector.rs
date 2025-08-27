use anyhow::Result;
use ratatui::crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph};
use std::collections::HashSet;
use std::fs;
use std::io::stdout;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_selected: bool,
}

pub struct FileSelector {
    base_path: PathBuf,
    current_path: PathBuf,
    items: Vec<FileItem>,
    list_state: ListState,
    selected_files: HashSet<PathBuf>,
    show_hidden: bool,
}

impl FileSelector {
    pub fn new(base_path: &Path) -> Self {
        let mut selector = Self {
            base_path: base_path.to_path_buf(),
            current_path: base_path.to_path_buf(),
            items: Vec::new(),
            list_state: ListState::default(),
            selected_files: HashSet::new(),
            show_hidden: false,
        };
        
        selector.refresh_items().unwrap_or(());
        selector.list_state.select(Some(0));
        selector
    }

    pub fn set_selections(&mut self, selections: Vec<PathBuf>) {
        self.selected_files = selections.into_iter().collect();
        self.update_item_selections();
    }

    fn refresh_items(&mut self) -> Result<()> {
        self.items.clear();
        
        // Add parent directory entry if not at base path
        if self.current_path != self.base_path {
            self.items.push(FileItem {
                path: self.current_path.parent().unwrap_or(&self.base_path).to_path_buf(),
                name: "..".to_string(),
                is_dir: true,
                is_selected: false,
            });
        }

        // Read directory contents
        let entries = fs::read_dir(&self.current_path)?;
        let mut items: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if !self.show_hidden {
                    !entry.file_name().to_string_lossy().starts_with('.')
                } else {
                    true
                }
            })
            .map(|entry| {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();
                let is_selected = !is_dir && self.selected_files.contains(&path);
                
                FileItem {
                    path,
                    name,
                    is_dir,
                    is_selected,
                }
            })
            .collect();

        // Sort: directories first, then files, both alphabetically
        items.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        self.items.extend(items);
        Ok(())
    }

    fn update_item_selections(&mut self) {
        for item in &mut self.items {
            if !item.is_dir {
                item.is_selected = self.selected_files.contains(&item.path);
            }
        }
    }

    pub fn run_interactive(&mut self) -> Result<Vec<PathBuf>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let result = self.run_event_loop(&mut terminal);

        // Cleanup
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;

        result
    }

    fn run_event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<Vec<PathBuf>> {
        loop {
            // Render the interface
            terminal.draw(|f| {
                self.render_ui(f);
            })?;

            // Handle events
            let event = read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Enter => {
                            if let Some(selected) = self.list_state.selected() {
                                if selected < self.items.len() {
                                    let item = &self.items[selected];
                                    if item.is_dir {
                                        self.current_path = item.path.clone();
                                        self.refresh_items()?;
                                        self.list_state.select(Some(0));
                                    } else {
                                        // Return selected files when Enter is pressed on a file
                                        return Ok(self.selected_files.iter().cloned().collect());
                                    }
                                }
                            }
                        }
                        KeyCode::Char(' ') => {
                            self.toggle_selection();
                        }
                        KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.select_all_files();
                        }
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.deselect_all();
                        }
                        KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.show_hidden = !self.show_hidden;
                            self.refresh_items()?;
                            self.list_state.select(Some(0));
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let selected = self.list_state.selected().unwrap_or(0);
                            if selected < self.items.len().saturating_sub(1) {
                                self.list_state.select(Some(selected + 1));
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            let selected = self.list_state.selected().unwrap_or(0);
                            if selected > 0 {
                                self.list_state.select(Some(selected - 1));
                            }
                        }
                        KeyCode::Char('r') => {
                            // Return/confirm selections
                            return Ok(self.selected_files.iter().cloned().collect());
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(self.selected_files.iter().cloned().collect())
    }

    fn render_ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(4),
            ])
            .split(f.area());

        // Create list items with visual indicators
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|item| {
                let mut style = Style::default();
                let prefix = if item.is_dir {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                    "📁"
                } else if item.is_selected {
                    style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
                    "✓ "
                } else {
                    style = style.fg(Color::White);
                    "  "
                };

                let display_name = if item.is_dir && item.name != ".." {
                    format!("{}/", item.name)
                } else {
                    item.name.clone()
                };

                ListItem::new(format!("{} {}", prefix, display_name))
                    .style(style)
            })
            .collect();

        // Create the list widget
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(format!("Files in: {}", self.current_path.display()))
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(list, chunks[0], &mut self.list_state.clone());

        // Render help and status
        let selected_count = self.selected_files.len();
        let help_text = format!(
            "Selected: {} files | SPACE=select | ENTER=enter dir/confirm | ↑↓=navigate | Ctrl+A=select all | Ctrl+D=clear | Ctrl+H=toggle hidden | R=confirm | Q=quit",
            selected_count
        );
        
        let status_paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Controls"))
            .style(Style::default().fg(Color::Yellow))
            .wrap(ratatui::widgets::Wrap { trim: true });
        
        f.render_widget(status_paragraph, chunks[1]);
    }

    fn toggle_selection(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.items.len() {
                let item = &mut self.items[selected];
                if !item.is_dir && item.name != ".." {
                    if item.is_selected {
                        self.selected_files.remove(&item.path);
                        item.is_selected = false;
                    } else {
                        self.selected_files.insert(item.path.clone());
                        item.is_selected = true;
                    }
                }
            }
        }
    }

    fn select_all_files(&mut self) {
        for item in &mut self.items {
            if !item.is_dir && item.name != ".." {
                self.selected_files.insert(item.path.clone());
                item.is_selected = true;
            }
        }
    }

    fn deselect_all(&mut self) {
        self.selected_files.clear();
        for item in &mut self.items {
            item.is_selected = false;
        }
    }
}