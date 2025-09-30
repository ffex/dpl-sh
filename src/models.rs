pub enum Status {
    Main,
    ChooseLang,
}

pub struct Language {
    pub name: String,
    pub code: String,
}

impl Language {
    pub fn new(name: String, code: String) -> Self {
        Language { name, code }
    }
    pub fn available_source_languages() -> Vec<Language> {
        vec![
            Language::new("Arabic".to_string(), "AR"),
            Language::new("Bulgarian".to_string(), "BG"),
            Language::new("Czech".to_string(), "CS"),
            Language::new("Danish".to_string(), "DA"),
            Language::new("German".to_string(), "DE"),
            Language::new("Greek".to_string(), "EL"),
            Language::new("English".to_string(), "EN"),
            Language::new("Spanish".to_string(), "ES"),
            Language::new("Estonian".to_string(), "ET"),
            Language::new("Finnish".to_string(), "FI"),
            Language::new("French".to_string(), "FR"),
            Language::new("Hebrew".to_string(), "HE"),
            Language::new("Hungarian".to_string(), "HU"),
            Language::new("Indonesian".to_string(), "ID"),
            Language::new("Italian".to_string(), "IT"),
            Language::new("Japanese".to_string(), "JA"),
            Language::new("Korean".to_string(), "KO"),
            Language::new("Lithuanian".to_string(), "LT"),
            Language::new("Latvian".to_string(), "LV"),
            Language::new("Norwegian (Bokmål)".to_string(), "NB"),
            Language::new("Dutch".to_string(), "NL"),
            Language::new("Polish".to_string(), "PL"),
            Language::new("Portuguese".to_string(), "PT"),
            Language::new("Romanian".to_string(), "RO"),
            Language::new("Russian".to_string(), "RU"),
            Language::new("Slovak".to_string(), "SK"),
            Language::new("Slovene".to_string(), "SL"),
            Language::new("Swedish".to_string(), "SV"),
            Language::new("Thai".to_string(), "TH"),
            Language::new("Turkish".to_string(), "TR"),
            Language::new("Ukrainian".to_string(), "UK"),
            Language::new("Vietnamese".to_string(), "VI"),
            Language::new("Chinese (Simplified)".to_string(), "ZH"),
        ]
    }

    pub fn available_target_languages() -> Vec<Language> {
        vec![
            Language::new("Arabic".to_string(), "AR"),
            Language::new("Bulgarian".to_string(), "BG"),
            Language::new("Czech".to_string(), "CS"),
            Language::new("Danish".to_string(), "DA"),
            Language::new("German".to_string(), "DE"),
            Language::new("Greek".to_string(), "EL"),
            Language::new("English".to_string(), "EN"),
            Language::new("English (United States)".to_string(), "EN-US"),
            Language::new("English (United Kingdom)".to_string(), "EN-GB"),
            Language::new("Spanish".to_string(), "ES"),
            Language::new(
                "Spanish (Latin America and the Caribbean)".to_string(),
                "ES-419",
            ),
            Language::new("Estonian".to_string(), "ET"),
            Language::new("Finnish".to_string(), "FI"),
            Language::new("French".to_string(), "FR"),
            Language::new("Hebrew".to_string(), "HE"),
            Language::new("Hungarian".to_string(), "HU"),
            Language::new("Indonesian".to_string(), "ID"),
            Language::new("Italian".to_string(), "IT"),
            Language::new("Japanese".to_string(), "JA"),
            Language::new("Korean".to_string(), "KO"),
            Language::new("Lithuanian".to_string(), "LT"),
            Language::new("Latvian".to_string(), "LV"),
            Language::new("Norwegian (Bokmål)".to_string(), "NB"),
            Language::new("Dutch".to_string(), "NL"),
            Language::new("Polish".to_string(), "PL"),
            Language::new("Portuguese".to_string(), "PT"),
            Language::new("Portuguese (Brazilian)".to_string(), "PT-BR"),
            Language::new("Portuguese (European)".to_string(), "PT-PT"),
            Language::new("Romanian".to_string(), "RO"),
            Language::new("Russian".to_string(), "RU"),
            Language::new("Slovak".to_string(), "SK"),
            Language::new("Slovene".to_string(), "SL"),
            Language::new("Swedish".to_string(), "SV"),
            Language::new("Thai".to_string(), "TH"),
            Language::new("Turkish".to_string(), "TR"),
            Language::new("Ukrainian".to_string(), "UK"),
            Language::new("Vietnamese".to_string(), "VI"),
            Language::new("Chinese (Simplified)".to_string(), "ZH-HANS"),
            Language::new("Chinese (Traditional)".to_string(), "ZH-HANT"),
        ]
    }
}
