use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

use crate::list_item::ListItem;
use std::{fs, path::PathBuf};

pub fn get_list_items(paths: fs::ReadDir) -> Vec<ListItem> {
    let mut items: Vec<ListItem> = paths
        .map(|path_result| {
            let path = &path_result.unwrap().path();
            ListItem::new(&path)
        })
        .collect();

    items.insert(0, ListItem::new(&PathBuf::from("../")));
    items
}

pub fn clean_current_path_line() {
    execute!(std::io::stdout(), cursor::MoveToPreviousLine(1)).unwrap();
    execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
}
