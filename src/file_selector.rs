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
    pub is_expanded: bool,
    pub depth: usize,
    pub parent_path: Option<PathBuf>,
}

pub struct FileSelector {
    base_path: PathBuf,
    items: Vec<FileItem>,
    list_state: ListState,
    selected_files: HashSet<PathBuf>,
    show_hidden: bool,
    expanded_dirs: HashSet<PathBuf>,
}

impl FileSelector {
    pub fn new(base_path: &Path) -> Self {
        let mut selector = Self {
            base_path: base_path.to_path_buf(),
            items: Vec::new(),
            list_state: ListState::default(),
            selected_files: HashSet::new(),
            show_hidden: false,
            expanded_dirs: HashSet::new(),
        };
        
        // Initially expand the base directory
        selector.expanded_dirs.insert(base_path.to_path_buf());
        selector.refresh_items().unwrap_or(());
        selector.list_state.select(Some(0));
        selector
    }

    pub fn set_selections(&mut self, selections: Vec<PathBuf>) {
        self.selected_files = selections.into_iter().collect();
        self.refresh_items().unwrap_or(());
    }

    fn refresh_items(&mut self) -> Result<()> {
        self.items.clear();
        let base_path = self.base_path.clone();
        self.build_tree(&base_path, 0, None)?;
        self.update_item_selections();
        Ok(())
    }

    fn build_tree(&mut self, dir_path: &Path, depth: usize, parent_path: Option<PathBuf>) -> Result<()> {
        // Read directory contents
        let entries = fs::read_dir(dir_path)?;
        let mut items: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if !self.show_hidden {
                    !entry.file_name().to_string_lossy().starts_with('.')
                } else {
                    true
                }
            })
            .collect();

        // Sort: directories first, then files, both alphabetically
        items.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        for entry in items {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = path.is_dir();
            let is_expanded = is_dir && self.expanded_dirs.contains(&path);
            let is_selected = !is_dir && self.selected_files.contains(&path);
            
            self.items.push(FileItem {
                path: path.clone(),
                name,
                is_dir,
                is_selected,
                is_expanded,
                depth,
                parent_path: parent_path.clone(),
            });

            // Recursively build tree for expanded directories
            if is_dir && is_expanded {
                self.build_tree(&path, depth + 1, Some(dir_path.to_path_buf()))?;
            }
        }

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
                            // ENTER always confirms selections
                            return Ok(self.selected_files.iter().cloned().collect());
                        }
                        KeyCode::Char(' ') => {
                            self.toggle_selection();
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            self.expand_current_directory();
                        }
                        KeyCode::Left => {
                            self.collapse_current_directory();
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

        // Create list items with visual indicators and tree structure
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|item| {
                let mut style = Style::default();
                
                // Create indentation based on depth
                let indent = "  ".repeat(item.depth);
                
                let (prefix, suffix) = if item.is_dir {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                    let expand_indicator = if item.is_expanded { "▼ " } else { "▶ " };
                    (format!("{}📁 {}", expand_indicator, item.name), "/".to_string())
                } else if item.is_selected {
                    style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
                    ("✓ ".to_string(), item.name.clone())
                } else {
                    style = style.fg(Color::White);
                    ("  ".to_string(), item.name.clone())
                };

                let display_text = if item.is_dir {
                    format!("{}{}{}", indent, prefix, suffix)
                } else {
                    format!("{}{}{}", indent, prefix, suffix)
                };

                ListItem::new(display_text).style(style)
            })
            .collect();

        // Create the list widget
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(format!("Files in: {}", self.base_path.display()))
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
            "Selected: {} files | SPACE=select/select dir | →=expand | ←=collapse | ENTER=confirm | ↑↓=navigate | Ctrl+A=select all | Ctrl+D=clear | Ctrl+H=toggle hidden | Q=quit",
            selected_count
        );
        
        let status_paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Controls"))
            .style(Style::default().fg(Color::Yellow))
            .wrap(ratatui::widgets::Wrap { trim: true });
        
        f.render_widget(status_paragraph, chunks[1]);
    }

    fn expand_current_directory(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.items.len() {
                let item = &self.items[selected];
                if item.is_dir && !item.is_expanded {
                    self.expanded_dirs.insert(item.path.clone());
                    self.refresh_items().unwrap_or(());
                }
            }
        }
    }

    fn collapse_current_directory(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.items.len() {
                let item = &self.items[selected];
                if item.is_dir && item.is_expanded {
                    self.expanded_dirs.remove(&item.path);
                    self.refresh_items().unwrap_or(());
                }
            }
        }
    }

    fn toggle_selection(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.items.len() {
                let item_path = self.items[selected].path.clone();
                let is_dir = self.items[selected].is_dir;
                
                if is_dir {
                    // Select all visible files in this directory
                    self.select_directory_files(&item_path);
                } else {
                    // Toggle individual file selection
                    if self.selected_files.contains(&item_path) {
                        self.selected_files.remove(&item_path);
                    } else {
                        self.selected_files.insert(item_path);
                    }
                    self.refresh_items().unwrap_or(());
                }
            }
        }
    }

    fn select_directory_files(&mut self, dir_path: &Path) {
        // Find all files in this directory (and subdirectories if expanded) and select them
        let files_to_select: Vec<PathBuf> = self.get_files_in_directory(dir_path);
        for file_path in files_to_select {
            self.selected_files.insert(file_path);
        }
        self.refresh_items().unwrap_or(());
    }

    fn get_files_in_directory(&self, dir_path: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                
                // Skip hidden files if not showing them
                if !self.show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                
                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() && self.expanded_dirs.contains(&path) {
                    // Recursively get files from expanded subdirectories
                    files.extend(self.get_files_in_directory(&path));
                }
            }
        }
        
        files
    }

    fn select_all_files(&mut self) {
        // Select all visible files in the entire tree
        for item in &self.items {
            if !item.is_dir {
                self.selected_files.insert(item.path.clone());
            }
        }
        self.refresh_items().unwrap_or(());
    }

    fn deselect_all(&mut self) {
        self.selected_files.clear();
        self.refresh_items().unwrap_or(());
    }
}