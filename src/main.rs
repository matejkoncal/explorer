use inquire::Select;
use std::env;
use std::fs;
use utils::{clean_current_path_line, get_list_items};

mod icons;
mod list_item;
mod utils;

fn main() -> Result<(), std::io::Error> {
    loop {
        let cwd = env::current_dir()?;
        let paths = fs::read_dir(&cwd)?;
        let items = get_list_items(paths);

        let select = Select::new(cwd.to_str().unwrap(), items)
            .with_page_size(20)
            .with_filter(&|input, _, item, _| {
                if item.ends_with("../") {
                    true
                } else {
                    item.to_lowercase().contains(&input)
                }
            });

        let answer = select.prompt().unwrap();

        if answer.path.is_file() {
            open::that(answer.path)?;
            break;
        } else {
            env::set_current_dir(answer.path)?;
            clean_current_path_line();
        }
    }

    Ok(())
}
