use crate::file_entry::{FileEntry, FileType};

pub struct Icons;

impl Icons {
    pub fn get_icon(entry: &FileEntry) -> &'static str {
        match entry.file_type {
            FileType::Directory => "",
            FileType::Symlink => "",
            FileType::File => Self::get_file_icon(entry),
        }
    }

    fn get_file_icon(entry: &FileEntry) -> &'static str {
        if let Some(ext) = entry.extension() {
            match ext.as_str() {
                // Programming languages
                "rs" => "",
                "py" => "",
                "js" => "",
                "ts" => "",
                "jsx" | "tsx" => "",
                "go" => "",
                "java" => "",
                "c" | "h" => "",
                "cpp" | "cc" | "cxx" | "hpp" => "",
                "cs" => "",
                "php" => "",
                "rb" => "",
                "swift" => "",
                "kt" => "",
                "lua" => "",
                "vim" => "",
                "sh" | "bash" | "zsh" => "",
                "fish" => "",
                "ps1" | "psm1" => "",

                // Web
                "html" | "htm" => "",
                "css" | "scss" | "sass" | "less" => "",
                "json" => "",
                "xml" => "",
                "yaml" | "yml" => "",
                "toml" => "",
                "md" | "markdown" => "",

                // Documents
                "pdf" => "",
                "doc" | "docx" => "",
                "xls" | "xlsx" => "",
                "ppt" | "pptx" => "",
                "txt" => "",

                // Images
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" | "webp" => "",

                // Videos
                "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" => "",

                // Audio
                "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "",

                // Archives
                "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => "",

                // Databases
                "db" | "sqlite" | "sql" => "",

                // Git
                "git" => "",
                "gitignore" | "gitattributes" | "gitmodules" => "",

                // Docker
                "dockerfile" => "",

                // Config files
                "conf" | "config" | "ini" | "env" => "",

                // Lock files
                "lock" => "",

                // Logs
                "log" => "",

                _ => "",
            }
        } else {
            // Special files without extensions
            match entry.name.to_lowercase().as_str() {
                "readme" | "readme.md" => "",
                "license" | "license.md" => "",
                "makefile" => "",
                "dockerfile" => "",
                "cargo.toml" => "",
                "package.json" => "",
                ".gitignore" => "",
                ".dockerignore" => "",
                ".env" => "",
                _ => "",
            }
        }
    }

    pub fn get_color(entry: &FileEntry) -> ratatui::style::Color {
        use ratatui::style::Color;

        match entry.file_type {
            FileType::Directory => Color::Blue,
            FileType::Symlink => Color::Cyan,
            FileType::File => {
                if let Some(ext) = entry.extension() {
                    match ext.as_str() {
                        "rs" | "go" | "c" | "cpp" | "java" | "py" | "js" | "ts" => {
                            Color::Yellow
                        }
                        "sh" | "bash" | "zsh" | "fish" | "ps1" => Color::Green,
                        "md" | "txt" | "pdf" | "doc" | "docx" => Color::White,
                        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => Color::Magenta,
                        "mp4" | "mkv" | "avi" | "mov" => Color::Magenta,
                        "mp3" | "wav" | "flac" => Color::Magenta,
                        "zip" | "tar" | "gz" | "7z" | "rar" => Color::Red,
                        _ => Color::White,
                    }
                } else {
                    Color::White
                }
            }
        }
    }
}