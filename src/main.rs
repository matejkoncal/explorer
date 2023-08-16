use std::env;
use utils::ask_user;
use utils::clean_current_path_line;

mod icons;
mod list_item;
mod utils;

fn main() -> Result<(), std::io::Error> {
    loop {
        let answer = ask_user()?;

        if answer.path.is_dir() {
            env::set_current_dir(answer.path)?;
            clean_current_path_line();
        } else {
            open::that(answer.path)?;
            break;
        }
    }

    Ok(())
}
