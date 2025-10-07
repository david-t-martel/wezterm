use crate::error::ExplorerError;
use crate::file_entry::{FileEntry, FileType};
use crate::git_status::GitStatus;
use crate::operations::FileOperation;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Search,
    Input(InputMode),
    Confirmation(ConfirmationMode),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Rename,
    New,
    Copy,
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfirmationMode {
    Delete,
}

pub struct App {
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub selected_entries: Vec<usize>,
    pub mode: AppMode,
    pub search_query: String,
    pub input_buffer: String,
    pub show_hidden: bool,
    pub show_preview: bool,
    pub git_status: Option<GitStatus>,
    pub scroll_offset: usize,
    pub error_message: Option<String>,
}

impl App {
    pub fn new(start_dir: PathBuf) -> Result<Self> {
        let mut app = Self {
            current_dir: start_dir.clone(),
            entries: Vec::new(),
            selected_index: 0,
            selected_entries: Vec::new(),
            mode: AppMode::Normal,
            search_query: String::new(),
            input_buffer: String::new(),
            show_hidden: false,
            show_preview: false,
            git_status: GitStatus::from_repo(&start_dir),
            scroll_offset: 0,
            error_message: None,
        };

        app.load_directory()?;
        Ok(app)
    }

    pub fn load_directory(&mut self) -> Result<()> {
        self.entries = FileEntry::read_directory(&self.current_dir, self.show_hidden)?;
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.git_status = GitStatus::from_repo(&self.current_dir);
        Ok(())
    }

    pub fn refresh_entries(&mut self) -> Result<()> {
        let current_selection = self.current_entry().map(|e| e.path.clone());
        self.entries = FileEntry::read_directory(&self.current_dir, self.show_hidden)?;

        // Try to restore selection
        if let Some(selected_path) = current_selection {
            if let Some(index) = self.entries.iter().position(|e| e.path == selected_path) {
                self.selected_index = index;
            }
        }

        self.git_status = GitStatus::from_repo(&self.current_dir);
        Ok(())
    }

    pub fn move_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1).min(self.entries.len() - 1);
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn go_top(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn go_bottom(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
        }
    }

    pub fn go_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            let _ = self.load_directory();
        }
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        if self.entries.is_empty() {
            return Ok(());
        }

        let entry = &self.entries[self.selected_index];
        if entry.file_type == FileType::Directory {
            self.current_dir = entry.path.clone();
            self.load_directory()?;
        }

        Ok(())
    }

    pub fn toggle_selection(&mut self) {
        if let Some(pos) = self
            .selected_entries
            .iter()
            .position(|&i| i == self.selected_index)
        {
            self.selected_entries.remove(pos);
        } else {
            self.selected_entries.push(self.selected_index);
        }
    }

    pub fn toggle_hidden_files(&mut self) -> Result<()> {
        self.show_hidden = !self.show_hidden;
        self.load_directory()
    }

    pub fn toggle_preview_pane(&mut self) {
        self.show_preview = !self.show_preview;
    }

    pub fn start_search(&mut self) {
        self.mode = AppMode::Search;
        self.search_query.clear();
    }

    pub fn start_delete_mode(&mut self) {
        if !self.entries.is_empty() {
            self.mode = AppMode::Confirmation(ConfirmationMode::Delete);
        }
    }

    pub fn start_rename_mode(&mut self) {
        if !self.entries.is_empty() {
            self.mode = AppMode::Input(InputMode::Rename);
            self.input_buffer = self.entries[self.selected_index]
                .name
                .clone();
        }
    }

    pub fn start_copy_mode(&mut self) {
        if !self.entries.is_empty() {
            self.mode = AppMode::Input(InputMode::Copy);
            self.input_buffer.clear();
        }
    }

    pub fn start_move_mode(&mut self) {
        if !self.entries.is_empty() {
            self.mode = AppMode::Input(InputMode::Move);
            self.input_buffer.clear();
        }
    }

    pub fn start_new_mode(&mut self) {
        self.mode = AppMode::Input(InputMode::New);
        self.input_buffer.clear();
    }

    pub fn is_confirmation_mode(&self) -> bool {
        matches!(self.mode, AppMode::Confirmation(_))
    }

    pub fn is_input_mode(&self) -> bool {
        matches!(self.mode, AppMode::Input(_))
    }

    pub fn handle_input(&mut self, c: char) {
        if matches!(self.mode, AppMode::Search) {
            self.search_query.push(c);
        } else if self.is_input_mode() {
            self.input_buffer.push(c);
        }
    }

    pub fn backspace_input(&mut self) {
        if matches!(self.mode, AppMode::Search) {
            self.search_query.pop();
        } else if self.is_input_mode() {
            self.input_buffer.pop();
        }
    }

    pub fn confirm_action(&mut self) -> Result<()> {
        match self.mode {
            AppMode::Confirmation(ConfirmationMode::Delete) => {
                self.delete_selected()?;
            }
            AppMode::Input(InputMode::Rename) => {
                self.rename_selected()?;
            }
            AppMode::Input(InputMode::New) => {
                self.create_new()?;
            }
            AppMode::Input(InputMode::Copy) => {
                self.copy_selected()?;
            }
            AppMode::Input(InputMode::Move) => {
                self.move_selected()?;
            }
            _ => {}
        }

        self.mode = AppMode::Normal;
        self.input_buffer.clear();
        Ok(())
    }

    fn delete_selected(&mut self) -> Result<()> {
        let indices = if self.selected_entries.is_empty() {
            vec![self.selected_index]
        } else {
            self.selected_entries.clone()
        };

        for &idx in indices.iter().rev() {
            if idx < self.entries.len() {
                FileOperation::delete(&self.entries[idx].path)?;
            }
        }

        self.selected_entries.clear();
        self.load_directory()?;
        Ok(())
    }

    fn rename_selected(&mut self) -> Result<()> {
        if !self.entries.is_empty() && !self.input_buffer.is_empty() {
            let old_path = &self.entries[self.selected_index].path;
            let new_path = old_path.parent().unwrap().join(&self.input_buffer);
            FileOperation::rename(old_path, &new_path)?;
            self.load_directory()?;
        }
        Ok(())
    }

    fn create_new(&mut self) -> Result<()> {
        if !self.input_buffer.is_empty() {
            let new_path = self.current_dir.join(&self.input_buffer);
            if self.input_buffer.ends_with('/') {
                FileOperation::create_directory(&new_path)?;
            } else {
                FileOperation::create_file(&new_path)?;
            }
            self.load_directory()?;
        }
        Ok(())
    }

    fn copy_selected(&mut self) -> Result<()> {
        if !self.entries.is_empty() && !self.input_buffer.is_empty() {
            let source = &self.entries[self.selected_index].path;
            let dest = self.current_dir.join(&self.input_buffer);
            FileOperation::copy(source, &dest)?;
            self.load_directory()?;
        }
        Ok(())
    }

    fn move_selected(&mut self) -> Result<()> {
        if !self.entries.is_empty() && !self.input_buffer.is_empty() {
            let source = &self.entries[self.selected_index].path;
            let dest = self.current_dir.join(&self.input_buffer);
            FileOperation::rename(source, &dest)?;
            self.load_directory()?;
        }
        Ok(())
    }

    pub fn get_selected_paths(&self) -> Option<Vec<PathBuf>> {
        if self.entries.is_empty() {
            return None;
        }

        if self.selected_entries.is_empty() {
            Some(vec![self.entries[self.selected_index].path.clone()])
        } else {
            Some(
                self.selected_entries
                    .iter()
                    .filter_map(|&idx| self.entries.get(idx).map(|e| e.path.clone()))
                    .collect(),
            )
        }
    }

    pub fn update(&mut self) -> Result<()> {
        // Update logic (e.g., watch for file system changes)
        Ok(())
    }

    pub fn visible_entries(&self) -> Vec<&FileEntry> {
        if self.search_query.is_empty() {
            self.entries.iter().collect()
        } else {
            self.entries
                .iter()
                .filter(|e| {
                    e.name
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
                })
                .collect()
        }
    }

    pub fn current_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }
}