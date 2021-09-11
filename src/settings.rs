pub struct Settings {
    pub length: u16,
    pub chars: String,
    pub entries: Vec<SettingEntry>
}

pub struct SettingEntry {
    pub name: String,
    pub description: String,
    pub toggled: bool,
    pub added_chars: String
}

macro_rules! col { // thanks Jvi
    ($($args:expr),*) => {{
        let mut result = String::new();
        $(
            result += &$args.collect::<String>();
        )*
        result
    }}
}

macro_rules! str {
    ($n:expr) => ($n.collect::<String>())
}

impl Settings {
    pub fn default() -> Self {
        Self {
            length: 16,
            chars: String::new(),
            entries: vec![
                SettingEntry::new("Include E-Symbols", "e.g. «©Æ‰Ñ", false, col!('‚'..='Œ', '‘'..='œ', '¡'..='ÿ')),
                SettingEntry::new("Include Symbols", "e.g. @#$%", false, col!('!'..='/', ':'..='@', '['..='`', '{'..='~')),
                SettingEntry::new("Include Numbers", "e.g. 12345", false, str!('0'..='9')),
                SettingEntry::new("Include Lowercase", "e.g. abc", false, str!('a'..='z')),
                SettingEntry::new("Include Uppercase", "e.g. ABC", false, str!('A'..='Z'))
            ]
        }
    }
}

impl SettingEntry {
    pub fn new(name: &str, description: &str, toggled: bool, added_chars: String) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            toggled,
            added_chars
        }
    }
}