use crate::icons::{get_icon_char, Icon};
use std::{fmt::Display, path::PathBuf};

pub struct ListItem {
    pub path: PathBuf,
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match self.path.extension() {
                Some(value) => get_icon_from_extension(value.to_str().unwrap()),
                None =>
                    if self.path.is_file() {
                        get_icon_char(Icon::File)
                    } else {
                        get_icon_char(Icon::Folder)
                    },
            },
            match self.path.file_name() {
                Some(name) => name.to_str().unwrap(),
                None => self.path.to_str().unwrap(),
            }
        )
    }
}

impl ListItem {
    pub fn new(path: &PathBuf) -> ListItem {
        ListItem {
            path: path.to_path_buf(),
        }
    }
}

fn get_icon_from_extension(extension: &str) -> char {
    match extension {
        "jsx" | "tsx" => get_icon_char(Icon::React),
        "js" => get_icon_char(Icon::Javascript),
        "ts" => get_icon_char(Icon::Typescript),
        "json" => get_icon_char(Icon::JSON),
        "html" => get_icon_char(Icon::HTML),
        "zip" => get_icon_char(Icon::Zip),
        "rs" => get_icon_char(Icon::Rust),
        _ => get_icon_char(Icon::File),
    }
}
