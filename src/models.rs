use std::fmt;
use std::str::FromStr;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageCode {
    AR,
    BG,
    CS,
    DA,
    DE,
    EL,
    EN,
    ENUS,
    ENGB,
    ES,
    ES419,
    ET,
    FI,
    FR,
    HE,
    HU,
    ID,
    IT,
    JA,
    KO,
    LT,
    LV,
    NB,
    NL,
    PL,
    PT,
    PTBR,
    PTPT,
    RO,
    RU,
    SK,
    SL,
    SV,
    TH,
    TR,
    UK,
    VI,
    ZH,
    ZHHANS,
    ZHHANT,
    Unknown(String),
}
impl fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            LanguageCode::AR => "AR",
            LanguageCode::BG => "BG",
            LanguageCode::CS => "CS",
            LanguageCode::DA => "DA",
            LanguageCode::DE => "DE",
            LanguageCode::EL => "EL",
            LanguageCode::EN => "EN",
            LanguageCode::ENUS => "EN-US",
            LanguageCode::ENGB => "EN-GB",
            LanguageCode::ES => "ES",
            LanguageCode::ES419 => "ES-419",
            LanguageCode::ET => "ET",
            LanguageCode::FI => "FI",
            LanguageCode::FR => "FR",
            LanguageCode::HE => "HE",
            LanguageCode::HU => "HU",
            LanguageCode::ID => "ID",
            LanguageCode::IT => "IT",
            LanguageCode::JA => "JA",
            LanguageCode::KO => "KO",
            LanguageCode::LT => "LT",
            LanguageCode::LV => "LV",
            LanguageCode::NB => "NB",
            LanguageCode::NL => "NL",
            LanguageCode::PL => "PL",
            LanguageCode::PT => "PT",
            LanguageCode::PTBR => "PT-BR",
            LanguageCode::PTPT => "PT-PT",
            LanguageCode::RO => "RO",
            LanguageCode::RU => "RU",
            LanguageCode::SK => "SK",
            LanguageCode::SL => "SL",
            LanguageCode::SV => "SV",
            LanguageCode::TH => "TH",
            LanguageCode::TR => "TR",
            LanguageCode::UK => "UK",
            LanguageCode::VI => "VI",
            LanguageCode::ZH => "ZH",
            LanguageCode::ZHHANS => "ZH-HANS",
            LanguageCode::ZHHANT => "ZH-HANT",
            LanguageCode::Unknown(s) => s,
        };
        write!(f, "{}", code)
    }
}

impl FromStr for LanguageCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AR" => Ok(LanguageCode::AR),
            "BG" => Ok(LanguageCode::BG),
            "CS" => Ok(LanguageCode::CS),
            "DA" => Ok(LanguageCode::DA),
            "DE" => Ok(LanguageCode::DE),
            "EL" => Ok(LanguageCode::EL),
            "EN" => Ok(LanguageCode::EN),
            "EN-US" => Ok(LanguageCode::ENUS),
            "EN-GB" => Ok(LanguageCode::ENGB),
            "ES" => Ok(LanguageCode::ES),
            "ES-419" => Ok(LanguageCode::ES419),
            "ET" => Ok(LanguageCode::ET),
            "FI" => Ok(LanguageCode::FI),
            "FR" => Ok(LanguageCode::FR),
            "HE" => Ok(LanguageCode::HE),
            "HU" => Ok(LanguageCode::HU),
            "ID" => Ok(LanguageCode::ID),
            "IT" => Ok(LanguageCode::IT),
            "JA" => Ok(LanguageCode::JA),
            "KO" => Ok(LanguageCode::KO),
            "LT" => Ok(LanguageCode::LT),
            "LV" => Ok(LanguageCode::LV),
            "NB" => Ok(LanguageCode::NB),
            "NL" => Ok(LanguageCode::NL),
            "PL" => Ok(LanguageCode::PL),
            "PT" => Ok(LanguageCode::PT),
            "PT-BR" => Ok(LanguageCode::PTBR),
            "PT-PT" => Ok(LanguageCode::PTPT),
            "RO" => Ok(LanguageCode::RO),
            "RU" => Ok(LanguageCode::RU),
            "SK" => Ok(LanguageCode::SK),
            "SL" => Ok(LanguageCode::SL),
            "SV" => Ok(LanguageCode::SV),
            "TH" => Ok(LanguageCode::TH),
            "TR" => Ok(LanguageCode::TR),
            "UK" => Ok(LanguageCode::UK),
            "VI" => Ok(LanguageCode::VI),
            "ZH" => Ok(LanguageCode::ZH),
            "ZH-HANS" => Ok(LanguageCode::ZHHANS),
            "ZH-HANT" => Ok(LanguageCode::ZHHANT),
            other => Ok(LanguageCode::Unknown(other.to_string())),
        }
    }
}

pub struct Language {
    pub name: &'static str,
    pub code: LanguageCode,
    pub available_as_source: bool,
    pub available_as_target: bool,
}

impl Language {
    pub fn available_source_languages() -> Vec<&'static Language> {
        LANGUAGES
            .iter()
            .filter(|lang| lang.available_as_source)
            .collect()
    }

    pub fn available_target_languages() -> Vec<&'static Language> {
        LANGUAGES
            .iter()
            .filter(|lang| lang.available_as_target)
            .collect()
    }

    pub fn from_name_case_insensitive(name: &str) -> Option<&'static Language> {
        LANGUAGES
            .iter()
            .find(|lang| lang.name.eq_ignore_ascii_case(name))
    }

    pub fn from_code(code: LanguageCode) -> &'static Language {
        LANGUAGES.iter().find(|lang| lang.code == code).unwrap()
    }
}
const LANGUAGES: &[Language] = &[
    Language {
        name: "Arabic",
        code: LanguageCode::AR,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Bulgarian",
        code: LanguageCode::BG,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Czech",
        code: LanguageCode::CS,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Danish",
        code: LanguageCode::DA,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "German",
        code: LanguageCode::DE,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Greek",
        code: LanguageCode::EL,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "English",
        code: LanguageCode::EN,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "English (United States)",
        code: LanguageCode::ENUS,
        available_as_source: false,
        available_as_target: true,
    },
    Language {
        name: "English (United Kingdom)",
        code: LanguageCode::ENGB,
        available_as_source: false,
        available_as_target: true,
    },
    Language {
        name: "Spanish",
        code: LanguageCode::ES,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Spanish (Latin America and the Caribbean)",
        code: LanguageCode::ES419,
        available_as_source: false,
        available_as_target: true,
    },
    Language {
        name: "Estonian",
        code: LanguageCode::ET,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Finnish",
        code: LanguageCode::FI,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "French",
        code: LanguageCode::FR,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Hebrew",
        code: LanguageCode::HE,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Hungarian",
        code: LanguageCode::HU,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Indonesian",
        code: LanguageCode::ID,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Italian",
        code: LanguageCode::IT,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Japanese",
        code: LanguageCode::JA,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Korean",
        code: LanguageCode::KO,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Lithuanian",
        code: LanguageCode::LT,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Latvian",
        code: LanguageCode::LV,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Norwegian (Bokmål)",
        code: LanguageCode::NB,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Dutch",
        code: LanguageCode::NL,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Polish",
        code: LanguageCode::PL,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Portuguese",
        code: LanguageCode::PT,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Portuguese (Brazilian)",
        code: LanguageCode::PTBR,
        available_as_source: false,
        available_as_target: true,
    },
    Language {
        name: "Portuguese (European)",
        code: LanguageCode::PTPT,
        available_as_source: false,
        available_as_target: true,
    },
    Language {
        name: "Romanian",
        code: LanguageCode::RO,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Russian",
        code: LanguageCode::RU,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Slovak",
        code: LanguageCode::SK,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Slovene",
        code: LanguageCode::SL,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Swedish",
        code: LanguageCode::SV,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Thai",
        code: LanguageCode::TH,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Turkish",
        code: LanguageCode::TR,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Ukrainian",
        code: LanguageCode::UK,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Vietnamese",
        code: LanguageCode::VI,
        available_as_source: true,
        available_as_target: true,
    },
    Language {
        name: "Chinese (Simplified)",
        code: LanguageCode::ZH,
        available_as_source: true,
        available_as_target: false,
    },
    Language {
        name: "Chinese (Simplified)",
        code: LanguageCode::ZHHANS,
        available_as_source: false,
        available_as_target: true,
    },
    Language {
        name: "Chinese (Traditional)",
        code: LanguageCode::ZHHANT,
        available_as_source: false,
        available_as_target: true,
    },
];
