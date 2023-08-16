use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use inquire::Select;

use crate::list_item::{ListItem, self};
use std::{fs, path::PathBuf, env};

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

pub fn filter_list_items<T>(input: &str, _: &T, item: &str, _: usize) -> bool {
    if item.ends_with("../") {
        true
    } else {
        item.to_lowercase().contains(&input)
    }
}

pub fn ask_user() -> Result<list_item::ListItem, std::io::Error> {
    let cwd = env::current_dir()?;
    let paths = fs::read_dir(&cwd)?;
    let items = get_list_items(paths);
    let select = Select::new(cwd.to_str().unwrap(), items)
        .with_page_size(20)
        .with_filter(&filter_list_items);
    let answer = select.prompt().unwrap();
    Ok(answer)
}
