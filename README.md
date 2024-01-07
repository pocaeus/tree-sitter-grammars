# tree-sitter-grammars

Repository containing tree-sitter grammars for many different languages available in one place, and a small CLI utility to add and update grammars.

## Contributing

If you'd like to add an additional language, please open a PR after following the instructions in [Adding a new grammar](#adding-a-new-grammar). Please take note the following conventions:

- Languages added to [`languages.toml`](./languages.toml) should ordered **alphabetically**
- Test the new grammar to make sure the parser is functional
- Replace an existing grammar **only** if the new grammar is more complete or better maintained
- Run `tree-sitter-grammars update` to add the grammar to [`grammars`](./grammars) directory
- Add a `hash` only if necessary (i.e. the latest commit is broken)

## Adding a new grammar

All languages are listed out in the [`languages.toml`](./languages.toml) file and are downloaded to the [`grammars`](./grammars) directory by default using the accompanying Rust program. To add a new language, modify the [`languages.toml`](./languages.toml) file to include:

```console
tree-sitter-grammars add --name foo \
                         --git git@github.com:user/tree-sitter-foo.git \
                         --hash 04937885edaae68e9b52001b88a6f72daeda391e \
                         --file languages.toml
```

Or alternatively you can manually add it to [`languages.toml`](./languages.toml):

```toml
foo = { name="tree-sitter-foo", git="git@github.com:user/tree-sitter-foo.git" }
# Optionally, you can add a specific commit hash
foo = { name="tree-sitter-foo", git="git@github.com:user/tree-sitter-foo.git", hash="04937885edaae68e9b52001b88a6f72daeda391e" }
```

Once added, run the updater to download the grammar:

```console
tree-sitter-grammars update --name foo \
                            --file languages.toml \
                            --dir grammars
```

## Updating all grammars

All listed grammars can be updated to their latest versions by running:

```console
tree-sitter-grammars update --all \
                            --file languages.toml \
                            --dir grammars
```

Take note of the `--all` flag. Alternatively, a single grammar is updated by providing the `--name` flag along with the language to be updated.

## License

Available under the MIT license. See [`LICENSE`](./LICENSE).
