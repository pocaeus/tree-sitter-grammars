//! This crate provides OCaml language support for the [tree-sitter][] parsing
//! library. There are separate languages for implementation, (`.ml`),
//! interfaces (`.mli`) and types.
//!
//! Typically, you will use the [language_ocaml][language func] function to add
//! this language to a tree-sitter [Parser][], and then use the parser to parse
//! some code:
//!
//! ```
//! let code = r#"
//!   module M = struct
//!     let x = 0
//!   end
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! parser
//!     .set_language(&tree_sitter_ocaml::language_ocaml())
//!     .expect("Error loading OCaml grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language_ocaml.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_ocaml() -> Language;
    fn tree_sitter_ocaml_interface() -> Language;
    fn tree_sitter_ocaml_type() -> Language;
}

/// Get the tree-sitter [Language][] for OCaml.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_ocaml() -> Language {
    unsafe { tree_sitter_ocaml() }
}

/// Get the tree-sitter [Language][] for OCaml interfaces.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_ocaml_interface() -> Language {
    unsafe { tree_sitter_ocaml_interface() }
}

/// Get the tree-sitter [Language][] for OCaml types.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_ocaml_type() -> Language {
    unsafe { tree_sitter_ocaml_type() }
}

/// The content of the [`node-types.json`][] file for OCaml.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const OCAML_NODE_TYPES: &'static str = include_str!("../../grammars/ocaml/src/node-types.json");

/// The content of the [`node-types.json`][] file for OCaml interfaces.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const INTERFACE_NODE_TYPES: &'static str = include_str!("../../grammars/interface/src/node-types.json");

/// The content of the [`node-types.json`][] file for OCaml types.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const TYPE_NODE_TYPES: &'static str = include_str!("../../grammars/type/src/node-types.json");

/// The syntax highlighting query for OCaml.
pub const HIGHLIGHTS_QUERY: &'static str = include_str!("../../queries/highlights.scm");

/// The local-variable syntax highlighting query for OCaml.
pub const LOCALS_QUERY: &'static str = include_str!("../../queries/locals.scm");

/// The symbol tagging query for OCaml.
pub const TAGGING_QUERY: &'static str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_ocaml() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::language_ocaml())
            .expect("Error loading OCaml grammar");

        let code = r#"
            module M = struct
              let x = 0
            end
        "#;

        let tree = parser.parse(code, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error());
    }

    #[test]
    fn test_ocaml_interface() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::language_ocaml_interface())
            .expect("Error loading OCaml interface grammar");

        let code = r#"
            module M : sig
              val x : int
            end
        "#;

        let tree = parser.parse(code, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error());
    }

    #[test]
    fn test_ocaml_type() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::language_ocaml_type())
            .expect("Error loading OCaml type grammar");

        let code = r#"int list"#;

        let tree = parser.parse(code, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error());
    }
}
