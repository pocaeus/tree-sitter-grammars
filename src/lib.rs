use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub git: String,
    pub hash: Option<String>,
}

impl Language {
    pub fn new(name: String, git: String, hash: Option<String>) -> Self {
        Self { name, git, hash }
    }
}

#[derive(Serialize, Deserialize)]
struct LanguageGrammarsTOML {
    languages: BTreeMap<String, Language>,
}

pub fn add_language_grammar_to_toml(name: String, language: Language, file_path: PathBuf) {
    let toml_contents = fs::read_to_string(&file_path).expect("Failed to read TOML file");
    let mut languages: LanguageGrammarsTOML =
        toml::from_str(&toml_contents).expect("Failed to parse TOML");

    if let Some(existing_language) = languages.languages.get_mut(&name) {
        if existing_language.hash != language.hash {
            existing_language.hash = language.hash;
        }
    } else {
        languages.languages.insert(name, language);
    }

    let comment =
        "# Automatically generated, DO NOT EDIT! Use `tree-sitter-grammars add` to modify.\n\n";

    let updated_toml = format!(
        "{}{}",
        comment,
        toml::to_string_pretty(&languages).expect("Failed to serialize to TOML")
    );
    fs::write(&file_path, updated_toml).expect("Failed to write updated TOML file");
}

pub fn update_language(name: Option<String>, all: bool, file_path: PathBuf, directory: PathBuf) {}
