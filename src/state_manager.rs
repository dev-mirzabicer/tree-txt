use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub selected_files: Vec<PathBuf>,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalState {
    pub projects: HashMap<String, ProjectState>,
}

pub struct StateManager {
    project_key: String,
    state_file: PathBuf,
}

impl StateManager {
    pub fn new(project_dir: &Path) -> Self {
        let project_key = project_dir
            .canonicalize()
            .unwrap_or_else(|_| project_dir.to_path_buf())
            .to_string_lossy()
            .to_string();

        let state_dir = if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("tree-txt")
        } else {
            PathBuf::from(".tree-txt")
        };

        let state_file = state_dir.join("state.toml");

        Self {
            project_key,
            state_file,
        }
    }

    pub fn load_selections(&self) -> Result<Vec<PathBuf>> {
        if !self.state_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.state_file)?;
        let global_state: GlobalState = toml::from_str(&content)?;

        if let Some(project_state) = global_state.projects.get(&self.project_key) {
            Ok(project_state.selected_files.clone())
        } else {
            Ok(Vec::new())
        }
    }

    pub fn save_selections(&mut self, selections: &[PathBuf]) -> Result<()> {
        if let Some(parent) = self.state_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut global_state = if self.state_file.exists() {
            let content = fs::read_to_string(&self.state_file)?;
            toml::from_str(&content).unwrap_or_default()
        } else {
            GlobalState::default()
        };

        let project_state = ProjectState {
            selected_files: selections.to_vec(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        };

        global_state
            .projects
            .insert(self.project_key.clone(), project_state);

        let content = toml::to_string_pretty(&global_state)?;
        fs::write(&self.state_file, content)?;

        Ok(())
    }
}
