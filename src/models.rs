use deepl::Lang;

pub const HELP_TEXT: &str = r#"
 q: Quit
 h: Toggle help
 i: Enter insert mode
 t: Traslate
 p: Paste from clipboard
 y: Copy to clipboard

"#;
//TODO add library to manage this?
pub enum Status {
    Main,
    ChooseLang,
}

pub enum Mode {
    Normal,
    Insert,
}
