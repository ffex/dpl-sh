use deepl::Lang;
//TODO rename this file? maybe utils?
pub const HELP_TEXT: &str = r#"
q: Quit
?: Toggle help
i: Enter insert mode
t: Translate
p: Paste from clipboard
y: Copy to clipboard
l: Choose language (then s for source, t for target)
s: Swap source and target languages
o: Clear text

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
