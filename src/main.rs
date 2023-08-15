use crate::list_item::ListItem;
use inquire::Select;
use std::env;
use std::fs;

mod icons;
mod list_item;

fn main() -> Result<(), std::io::Error> {
    let cwd = env::current_dir()?;
    let paths = fs::read_dir(&cwd)?;

    let items = paths
        .map(|path_result| {
            let path = &path_result.unwrap().path();
            ListItem::new(&path)
        })
        .collect();

    let mut select = Select::new(cwd.to_str().unwrap(), items);
    select.page_size = 50;
    let _ans = select.prompt();

    Ok(())
}
