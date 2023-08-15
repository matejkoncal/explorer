pub enum Icon {
    Folder = 0xe5ff,
    React = 0xe7ba,
    Javascript = 0xe74e,
    Typescript = 0xe628,
    JSON = 0xe60b,
    HTML = 0xe60e,
    File = 0xea7b,
    Zip = 0xe6aa,
    Rust = 0xe7a8,
}

pub fn get_icon_char(icon: Icon) -> char {
    char::from_u32(icon as u32).unwrap()
}
