# tree-sitter-grammars

Repository containing tree-sitter grammars for every language available in one place.

## Grammars

All languages are listed out in the [`languages.toml`](./languages.toml) file and are downloaded to the [`grammars`](./grammars) directory using the accompanying Rust program. To add a new language, modify the [`languages.toml`](./languages.toml) file to include:

```toml
foo = { name="tree-sitter-foo", git="git@github.com:user/tree-sitter-foo.git" }
# Optionally, you can add a specific commit hash
foo = { name="tree-sitter-foo", commit="04937885edaae68e9b52001b88a6f72daeda391e", git="git@github.com:user/tree-sitter-foo.git" }
```

Once added, run the updater to download the grammar:

```shell
tree-sitter-grammars --update -f languages.toml -d grammars
```

## License

Available under the MIT license. See [`LICENSE`](./LICENSE).