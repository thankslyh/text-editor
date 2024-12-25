use std::{
    fs::{self, File},
    io::Error,
    io::Write,
};

use crate::{fileinfo::FileInfo, line::Line, view::Location};

#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_info: FileInfo,
    pub is_modify: bool,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            lines: Vec::new(),
            file_info: FileInfo { path: None },
            is_modify: false,
        }
    }
}

impl Buffer {
    pub fn read_file(filepath: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filepath)?;
        let mut lines = Vec::new();
        for str in contents.lines() {
            lines.push(Line::from(str));
        }
        Ok(Self {
            lines,
            file_info: FileInfo::from(filepath),
            is_modify: false,
        })
    }

    pub fn insert_char(&mut self, s: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&s.to_string()));
            self.is_modify = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.intert_char(s, at.grapheme_index);
            self.is_modify = true;
        }
    }

    pub fn delete(&mut self, at: Location) {
        let Location {
            line_index,
            grapheme_index,
        } = at;
        if let Some(line) = self.lines.get(line_index) {
            // 从非最后一行的行末删除
            if at.grapheme_index >= line.len() && self.height() > line_index.saturating_add(1) {
                let next_line = self.lines.remove(line_index.saturating_add(1));
                self.lines[line_index].append(&next_line);
                self.is_modify = true;
            } else if grapheme_index < line.len() {
                self.lines[line_index].delete(grapheme_index);
                self.is_modify = true;
            }
        }
    }

    pub fn insert_new_line(&mut self, at: Location) {
        if at.line_index == self.height() {
            self.lines.push(Line::default());
            self.is_modify = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new_line = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new_line);
            self.is_modify = true;
        }
    }

    pub fn save_to_file(&self, fileinfo: &FileInfo) -> Result<(), Error> {
        if let Some(path) = fileinfo.get_path() {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }

    pub fn save_as(&mut self, filename: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(filename);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.is_modify = false;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.save_to_file(&self.file_info)?;
        self.is_modify = false;
        Ok(())
    }

    pub fn is_file_loaded(&self) -> bool {
        self.file_info.has_path()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }
}
