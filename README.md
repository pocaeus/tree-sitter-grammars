# tree-sitter-grammars

Repository containing tree-sitter grammars for every language available in one place.

## Contributing

If you'd like to add an additional language, please open a PR after following the instructions in [Adding a new grammar](#adding-a-new-grammar). Please take note the following conventions:

- Languages added to [`languages.toml`](./languages.toml) should ordered **alphabetically**
- Replace an existing grammar **only** if the new grammar is more complete or better maintained
- Make sure `tree-sitter-grammars update` is run to add the grammar to [`grammars`](./grammars) directory
- Add a `hash` only if necessary (i.e. the latest commit is broken)

## Adding a new grammar

All languages are listed out in the [`languages.toml`](./languages.toml) file and are downloaded to the [`grammars`](./grammars) directory by default using the accompanying Rust program. To add a new language, modify the [`languages.toml`](./languages.toml) file to include:

```console
tree-sitter-grammars add --name foo \
    --git git@github.com:user/tree-sitter-foo.git \
    --hash 04937885edaae68e9b52001b88a6f72daeda391e \  # optional
    --file languages.toml  # this will modify the `languages.toml` file to add foo to the list
```

Or alternatively you can manually add it to [`languages.toml`](./languages.toml):

```toml
foo = { name="tree-sitter-foo", git="git@github.com:user/tree-sitter-foo.git" }
# Optionally, you can add a specific commit hash
foo = { name="tree-sitter-foo", git="git@github.com:user/tree-sitter-foo.git", hash="04937885edaae68e9b52001b88a6f72daeda391e" }
```

Once added, run the updater to download the grammar:

```console
tree-sitter-grammars update --file languages.toml --dir grammars
```

## License

Available under the MIT license. See [`LICENSE`](./LICENSE).