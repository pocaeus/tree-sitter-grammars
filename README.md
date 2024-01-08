# tree-sitter-grammars

Repository containing tree-sitter grammars for many different languages available in one place, and a small CLI utility to add and update grammars.

## Usage & Development

```console
Command line program to add and update tree-sitter grammars for different languages.

Usage: tree-sitter-grammars [OPTIONS] [COMMAND]

Commands:
  add     Add a new tree-sitter grammar to the `languages.toml` file
  update  Update the tree-sitter grammar(s)
  help    Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>            Path to file containing languages and their grammar repositories [default: ./languages.toml]
  -d, --directory <DIRECTORY>  Path to directory containing grammar repositories [default: ./grammars/]
  -h, --help                   Print help
  -V, --version                Print version
```

If you have nix installed you can simply run `nix build`, or alternatively use the development shell with `nix develop` and then run `cargo build`.

## Contributing

If you'd like to add an additional language, please open a PR after following the instructions in [Adding a new grammar](#adding-a-new-grammar). Please take note the following conventions:

- Languages added to [`languages.toml`](./languages.toml) should ordered **alphabetically**
- Test the new grammar to make sure the parser is functional
- Replace an existing grammar **only** if the new grammar is more complete or better maintained
- Run `tree-sitter-grammars update` to add the grammar to [`grammars`](./grammars) directory
- Add a `hash` only if necessary (i.e. the latest commit is broken)

### Adding a new grammar

All languages are listed out in the [`languages.toml`](./languages.toml) file and are downloaded to the [`grammars`](./grammars) directory by default using the accompanying Rust program. To add a new language, modify the [`languages.toml`](./languages.toml) file to include:

```console
tree-sitter-grammars add --name foo \
                         --git git@github.com:user/tree-sitter-foo.git \
                         --hash 04937885edaae68e9b52001b88a6f72daeda391e \
```

Once added, run the updater to download the grammar:

```console
tree-sitter-grammars update --name foo
```

### Updating all grammars

All listed grammars can be updated to their latest versions by running:

```console
tree-sitter-grammars update --all
```

Take note of the `--all` flag. Alternatively, a single grammar is updated by providing the `--name` flag along with the language to be updated.

## License

Available under the MIT license. See [`LICENSE`](./LICENSE).
