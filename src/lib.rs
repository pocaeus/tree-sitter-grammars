use git2::Oid;
use git2::Repository;
use num_cpus;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::thread;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub fn update_language(
    name: Option<String>,
    all: bool,
    file_path: PathBuf,
    directory: PathBuf,
) -> () {
    if let Some(language_name) = name {
        let toml_contents = fs::read_to_string(&file_path).expect("Failed to read TOML file");
        let languages: LanguageGrammarsTOML =
            toml::from_str(&toml_contents).expect("Failed to parse TOML");

        if let Some(language) = languages.languages.get(&language_name) {
            let destination_directory = format!("{}{}", directory.display(), &language.name);
            if let Err(err) = clone_repository(language.clone(), destination_directory) {
                eprintln!("Cloning error: {:?}", err);
            }
        } else {
            eprintln!("Language not found: {}", language_name);
        }
    } else if all {
        println!("Updating all languages");
        let toml_contents = fs::read_to_string(&file_path).expect("Failed to read TOML file");
        let languages: LanguageGrammarsTOML =
            toml::from_str(&toml_contents).expect("Failed to parse TOML");

        let tasks: Vec<_> = languages
            .languages
            .iter()
            .map(|(_, language)| {
                let destination_directory = format!("{}{}", directory.display(), &language.name);
                (language.clone(), destination_directory)
            })
            .collect();

        let handles: Vec<_> = tasks
            .into_iter()
            .map(|(language, destination_directory)| {
                thread::spawn(move || {
                    if let Err(err) = clone_repository(language.clone(), destination_directory) {
                        eprintln!("Cloning error: {:?}", err);
                    }
                })
            })
            .collect();

        for handle in handles {
            if let Err(err) = handle.join() {
                eprintln!("Thread join error: {:?}", err);
            }
        }
    } else {
        eprintln!("Please provide a language name or use the --all option.");
    }
}

fn clone_repository(language: Language, directory: String) -> Result<(), String> {
    if let Err(e) = fs::remove_dir_all(&directory) {
        if e.kind() != std::io::ErrorKind::NotFound {
            eprintln!("Failed to remove existing directory: {:?}", e);
        }
    }
    match Repository::clone(&language.git, Path::new(&directory)) {
        Ok(repo) => {
            if let Some(commit_hash) = language.hash {
                let _ = repo.set_head_detached(
                    Oid::from_str(&commit_hash)
                        .ok()
                        .expect("Failed to checkout the specific commit"),
                );
            }
            println!("Updated language: {:?}", language.name);
            return Ok(());
        }
        Err(e) => {
            return Err(format!(
                "Failed to update language: {:?} (error: {:?})",
                language.name,
                e.message()
            ));
        }
    }
}
